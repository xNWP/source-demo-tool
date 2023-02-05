pub mod frame;
pub mod header;
pub mod packet;

use std:: {
    fs::File,
    vec::Vec,
    io::{Read, Seek, SeekFrom},
    thread, path::PathBuf,
};

use async_channel::{self, Receiver, Sender};
use buf_redux::BufReader;

//use kdam::{tqdm, BarExt};

use self::{
    frame::{ Frame, FrameIndex, DataTablesData, Command, },
    header::DemoHeader,
    packet::netmessage::{
        NetMessage,
        GameEventListData,
        ServerInfoData,
        UserMessageData,
    },
    packet::usermessage::{
        UserMessage,
    }
};

#[derive(Debug, Clone)]
pub struct DemoFile {
    pub path: PathBuf,
    pub header: DemoHeader,
    pub frames: Vec<frame::Frame>,
    pub sign_on_frames: Vec<Frame>
}

impl DemoFile {
    pub fn open(filepath: &PathBuf) -> Result<Self, String> {
        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Err(format!("file '{}' not found", filepath.to_str().unwrap())),
                _ => return Err(String::from("an unexpected file error occured"))
            }
        };

        let mut reader = BufReader::new(file);

        // read header
        let header = match DemoHeader::from_readable(&mut reader) {
            Ok(h) => h,
            Err(e) => return Err(format!("couldn't parse header: {e}"))
        };

        // read sign on data
        let mut sign_on_data: Vec<u8> = Vec::new();
        sign_on_data.resize(header.sign_on_length.try_into().unwrap(), 0);
        match reader.read_exact(sign_on_data.as_mut_slice()) {
            Ok(()) => {},
            Err(_) => return Err(String::from("couldn't read sign_on_data"))
        }

        // parse sign on data
        let mut sod_reader = BufReader::with_capacity(sign_on_data.len(), sign_on_data.as_slice());
        sod_reader.read_into_buf().unwrap();
        let mut sign_on_frames = Vec::new();

        while sod_reader.buf_len() > 0 {
            let frame_index = match FrameIndex::from_readable(&mut sod_reader) {
                Ok(fi) => fi,
                Err(e) => return Err(format!(
                    "invalid or corrupt sign_on_data: {:?}", e
                ))
            };

            sign_on_frames.push(Frame::from_frame_index(frame_index)?);
        }

        // index frames

        // we'll index frames on the main thread, while worker threads parse the frames
        //    main thread: index frames and send to worker threads, receive completed work
        // worker threads: parse frames and send results back to main thread

        // setup data channel
        let (tx_to_workers, rx_from_main) = async_channel::unbounded();
        let (tx_to_main, rx_from_workers) = async_channel::unbounded();

        // build and spawn worker threads
        let core_count: usize = thread::available_parallelism()
            .expect("couldn't get available cores")
            .into();

            let mut worker_threads = Vec::new();
            for i in 0..core_count {
                let rx = rx_from_main.clone();
                let tx = tx_to_main.clone();
                match thread::Builder::new()
                .name(format!("frame parser thread {i}"))
                .spawn(move || Self::worker_thread_receive_parse_and_send(rx, tx)) {
                    Ok(h) => worker_threads.push(h),
                    Err(e) => return Err(format!("couldn't spawn frame worker thread[{i}]: {e}"))
                };
            }
        drop(tx_to_main);
        drop(rx_from_main);

        Self::index_and_send_frames(reader, &tx_to_workers)?;
        drop(tx_to_workers);

        let frames = match Self::receive_parsed_frames(&rx_from_workers){
            Ok(f) => f,
            Err(e) => return Err(e.into())
        };
        drop(rx_from_workers);

        for join_handle in worker_threads {
            join_handle.join().unwrap();
        }

        Ok(
            DemoFile {
                frames,
                header,
                sign_on_frames,
                path: filepath.clone(),
            }
        )
    }

    fn receive_parsed_frames(rx: &Receiver<Result<Frame, &'static str>>) -> Result<Vec<Frame>, &'static str> {
        let mut frames = Vec::new();

        // receive frames (out of order)
        loop {
            match rx.recv_blocking() {
                Ok(frame) => {
                    frames.push(frame?);
                    //pb.update(1);
                },
                Err(_) => break
            };
        }

        // order frames and return
        frames.sort_unstable_by_key(|f| f.tick);
        Ok(frames)
    }

    fn index_and_send_frames(mut reader: impl Read + Seek, tx: &Sender<FrameIndex>) -> Result<(), String> {
        let current_location = reader.stream_position().unwrap();
        let file_length = match reader.seek(SeekFrom::End(0)) {
            Ok(n) => n,
            Err(_) => return Err(String::from("couldn't get eof"))
        };
        reader.seek(SeekFrom::Start(current_location)).unwrap();

        loop {
            let file_position = reader.stream_position().unwrap();

            if file_position >= file_length {
                if file_position != file_length {
                    return Err(String::from("bad frame index read"))
                }
                return Ok(())
            }

            let frame_index = match FrameIndex::from_readable(&mut reader) {
                Ok(fi) => fi,
                Err(e) => return Err(format!("error occured parsing frame index: {:?}", e))
            };

            if let Err(e) = tx.send_blocking(frame_index) {
                return Err(format!("error occured sending frame index: {e}"))
            }
        }
    }

    fn worker_thread_receive_parse_and_send(rx: Receiver<FrameIndex>, tx: Sender<Result<Frame, &'static str>>) {
        loop {
            let frame_index = match rx.recv_blocking() {
                Ok(f) => f,
                Err(_) => return
            };

            // parse and send data
            let frame = frame::Frame::from_frame_index(frame_index);
            tx.send_blocking(frame).unwrap()
        }
    }

    pub fn get_data_tables(self: &Self) -> Vec<&DataTablesData> {
        let mut dtables = Vec::new();
        for f in &self.sign_on_frames {
            match &f.command {
                Command::DataTables(dt) => dtables.push(dt),
                _ => {}
            }
        }
        dtables
    }

    pub fn get_server_info(self: &Self) -> Option<&ServerInfoData> {
        for f in &self.sign_on_frames {
            match &f.command {
                Command::SignOn(pd) => {
                    for msg in &pd.network_messages {
                        if let Some(nmsg) = &msg.message {
                            match nmsg {
                                NetMessage::ServerInfo(si) => return Some(si),
                                _ => continue
                            }
                        }
                    }
                },
                _ => continue
            }
        }
        None
    }

    fn get_game_event_list(self: &Self) -> Option<&GameEventListData> {
        for f in &self.sign_on_frames {
            match &f.command {
                Command::SignOn(pd) => {
                    for msg in &pd.network_messages {
                        if let Some(nmsg) = &msg.message {
                            match nmsg {
                                NetMessage::GameEventList(gel) => return Some(gel),
                                _ => continue
                            }
                        }
                    }
                },
                _ => continue
            }
        }
        None
    }

    pub fn get_full_game_events(self: &Self) -> Vec<FullGameEvent> {
        let mut events = Vec::new();

        let game_event_list = match self.get_game_event_list() {
            Some(gel) => gel,
            None => return events
        };

        for f in &self.frames {
            match &f.command {
                Command::Packet(pd) => {
                    for msg in &pd.network_messages {
                        if let Some(nmsg) = &msg.message {
                            match &nmsg {
                                NetMessage::GameEvent(ge) => {
                                    let event_id = ge.event_id.unwrap();
                                    let mut event_listing = None;

                                    // find the game event listing
                                    for listing in &game_event_list.Descriptors {
                                        if event_id == listing.event_id.unwrap() {
                                            event_listing = Some(listing);
                                            break;
                                        }
                                    }

                                    let event_listing = event_listing.unwrap();
                                    assert_eq!(event_listing.DescriptorKeys.len(), ge.GameEventKeys.len());

                                    let mut event_keys = Vec::new();
                                    for i in 0..ge.GameEventKeys.len() {
                                        let listing = &event_listing.DescriptorKeys[i];
                                        let ev = &ge.GameEventKeys[i];
                                        assert_eq!(listing.key_type.unwrap(), ev.val_type.unwrap());

                                        let key_type: FullGameEventKeyType = listing.key_type.unwrap().try_into().unwrap();
                                        let key_name = listing.key_name.as_ref().unwrap().clone();

                                        let mut val_string = None;
                                        let mut val_float = None;
                                        let mut val_int = None;
                                        let mut val_bool = None;

                                        match key_type {
                                            FullGameEventKeyType::String => val_string = ev.val_string.clone(),
                                            FullGameEventKeyType::Float => val_float = ev.val_float.clone(),
                                            FullGameEventKeyType::Long => val_int = ev.val_long.clone(),
                                            FullGameEventKeyType::Byte => val_int = ev.val_byte.clone(),
                                            FullGameEventKeyType::Short => val_int = ev.val_short.clone(),
                                            FullGameEventKeyType::Bool => val_bool = Some(*ev.val_bool.as_ref().unwrap() > 0)
                                        }

                                        event_keys.push(FullGameEventKey{
                                            key_name,
                                            key_type,
                                            val_string,
                                            val_float,
                                            val_int,
                                            val_bool
                                        });
                                    }
                                    let event_name = event_listing.name.as_ref().unwrap().clone();
                                    let event_tick = f.tick;
                                    events.push( FullGameEvent { event_id, event_name, event_keys, event_tick })
                                },
                                _ => continue
                            }
                        };
                    }
                },
                _ => continue
            }
        }

        events
    }


    fn get_all_user_message_data(self: &Self) -> Vec<(usize, usize, i32, &UserMessageData)> {
        let mut user_message_data = Vec::new();

        for it in 0..self.frames.len() {
            let f = &self.frames[it];
            let tick = f.tick;
            if let Command::Packet(pd) = &f.command {
                for jt in 0..pd.network_messages.len() {
                    let nmsg = &pd.network_messages[jt];
                    let msg = nmsg.message.as_ref().unwrap();
                    match msg {
                        NetMessage::UserMessage(umd) => {
                            user_message_data.push((it, jt, tick, umd));
                        },
                        _ => continue
                    }
                }
            }
        }

        user_message_data
    }

    pub fn get_user_messages(self: &Self) -> Vec<ParsedUserMessage> {
        let mut user_messages = Vec::new();

        let all_user_message_data = self.get_all_user_message_data();

        for msg in all_user_message_data {
            let data = msg.3.msg_data.as_ref().unwrap();
            let mut reader = BufReader::with_capacity(data.len(), data.as_slice());
            reader.read_into_buf().unwrap();

            match UserMessage::parse_from_id_and_bufredux_reader(msg.3.msg_type.unwrap(), &mut reader) {
                Ok((inner_msg, warns)) => user_messages.push( ParsedUserMessage {
                    frame_index: msg.0,
                    message_index: msg.1,
                    tick: msg.2,
                    message_return: packet::MessageParseReturn {
                        message: Some(inner_msg),
                        warnings: Some(warns),
                        err: None,
                    }
                }),
                Err(e) => user_messages.push( ParsedUserMessage {
                    frame_index: msg.0,
                    message_index: msg.1,
                    tick: msg.2,
                    message_return: packet::MessageParseReturn {
                        message: None,
                        warnings: None,
                        err: Some(e)
                    }
                })
            }
        }

        user_messages
    }
}

#[derive(Debug, Clone)]
pub struct ParsedUserMessage {
    pub frame_index: usize,
    pub message_index: usize,
    pub message_return: packet::MessageParseReturn<UserMessage>,
    pub tick: i32,
}

#[derive(Debug, Clone)]
pub struct FullGameEvent {
    pub event_name: String,
    pub event_id: u64,
    pub event_tick: i32,
    pub event_keys: Vec<FullGameEventKey>
}

#[derive(Debug, Clone)]
pub struct FullGameEventKey {
    pub   key_name: String,
    pub   key_type: FullGameEventKeyType,
    pub val_string: Option<String>,
    pub  val_float: Option<f32>,
    pub    val_int: Option<u64>,
    pub   val_bool: Option<bool>
}

#[derive(Debug, Clone)]
pub enum FullGameEventKeyType {
    String,
    Float,
    Long,
    Short,
    Byte,
    Bool
}

impl TryFrom<u64> for FullGameEventKeyType {
    type Error = &'static str;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FullGameEventKeyType::String),
            2 => Ok(FullGameEventKeyType::Float),
            3 => Ok(FullGameEventKeyType::Long),
            4 => Ok(FullGameEventKeyType::Short),
            5 => Ok(FullGameEventKeyType::Byte),
            6 => Ok(FullGameEventKeyType::Bool),
            _ => Err("unknown key type")
        }
    }
}