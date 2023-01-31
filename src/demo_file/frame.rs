use std::io::Read;

use crate:: {
    demo_file::packet::{
        netmessage::{ NetMessage, SendTableData },
        PacketData, PacketIndex,
    },
    event_data::EventData,
    parse_tools::{ parse_cstr, parse_i32, parse_u8, parse_u16, parse_u32, },
};

use source_demo_tool_impl_proc_macros::event;

use buf_redux::BufReader;

#[derive(Debug, Clone)]
pub struct Frame {
    pub command: Command,
    pub tick: i32,
    pub player_slot: u8,
}

impl Frame {
    pub fn from_frame_index(frame_index: FrameIndex) -> Result<Self, &'static str> {
        let command = frame_index.command_index.try_into()?;
        let tick = frame_index.tick;
        let player_slot = frame_index.player_slot;

        Ok(Self {
            command,
            tick,
            player_slot,
        })
    }
}

pub struct FrameIndex {
    pub command_index: CommandIndex,
    pub tick: i32,
    pub player_slot: u8,
}

impl FrameIndex {
    pub fn from_readable(mut reader: impl Read) -> Result<Self, EventData> {
        let command_num = match parse_u8(&mut reader) {
            Ok(n) => n,
            Err(_e) => return Err(event!{"couldn't parse command"})
        };

        let tick = match parse_i32(&mut reader) {
            Ok(n) => n,
            Err(_e) => return Err(event!{"couldn't parse tick"})
        };

        let player_slot = match parse_u8(&mut reader) {
            Ok(n) => n,
            Err(_e) => return Err(event!{"couldn't parse player slot"})
        };

        let command_index = match CommandIndex::from_u8_and_reader(command_num, &mut reader) {
            Ok(ci) => ci,
            Err(e) => return Err(e)
        };

        Ok( Self {
            command_index,
            tick,
            player_slot,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    SignOn(PacketData),
    Packet(PacketData),
    SyncTick,
    //ConsoleCmd,
    //UserCmd,
    DataTables(DataTablesData),
    Stop,
    //StringTables
}

impl TryFrom<CommandIndex> for Command {
    type Error = &'static str;
    fn try_from(value: CommandIndex) -> Result<Self, Self::Error> {
        match value {
            CommandIndex::Packet(pi)      => Ok(Command::Packet(PacketData::from_packet_index(pi))),
            CommandIndex::SignOn(pi)      => Ok(Command::SignOn(PacketData::from_packet_index(pi))),
            CommandIndex::SyncTick        => Ok(Command::SyncTick),
            CommandIndex::DataTables(dti) => Ok(Command::DataTables(DataTablesData::from_data_tables_index(dti)?)),
            CommandIndex::Stop => Ok(Command::Stop),
        }
    }
}

impl Command {
    pub fn as_u8(&self) -> u8 {
        match self {
            Command::SignOn(_)     => command_id::SIGN_ON,
            Command::Packet(_)     => command_id::PACKET,
            Command::SyncTick      => command_id::SYNC_TICK,
            //Command::ConsoleCmd   => command_id::ConsoleCmd,
            //Command::UserCmd      => command_id::UserCmd,
            Command::DataTables(_) => command_id::DATA_TABLES,
            Command::Stop          => command_id::STOP,
            //Command::StringTables => command_id::StringTables,
        }
    }

    pub fn get_command_str(&self) -> &str {
        match self {
            Command::SignOn(_)     =>     "SignOn",
            Command::Packet(_)     =>     "Packet",
            Command::SyncTick      =>   "SyncTick",
            //Command::ConsoleCmd   => command_id::ConsoleCmd,
            //Command::UserCmd      => command_id::UserCmd,
            Command::DataTables(_) => "DataTables",
            Command::Stop          =>       "Stop",
        }
    }
}

pub struct DataTablesIndex {
    pub data: Vec<u8>
}

impl DataTablesIndex {
    pub fn from_readable(mut reader: impl Read) -> Result<Self, String> {
        let data_len = match parse_u32(&mut reader) {
            Ok(n) => n,
            Err(e) => return Err(format!{"parsing data_len: {}", e})
        };

        let mut data = Vec::new();
        data.resize(data_len.try_into().unwrap(), 0);
        reader.read_exact(data.as_mut_slice()).unwrap();

        Ok(Self { data })
    }
}

#[derive(Debug, Clone)]
pub struct DataTablesData {
    pub send_tables: Vec<SendTableData>,
    pub class_descriptions: Vec<DataTablesClassDescription>,
}

#[derive(Debug, Clone)]
pub struct DataTablesClassDescription {
    pub     class_id: u16,
    pub network_name: String,
    pub   table_name: String,
}

impl DataTablesData {
    pub fn from_data_tables_index(dti: DataTablesIndex) -> Result<Self, &'static str> {
        let mut data_reader = BufReader::with_capacity
            (dti.data.len(), dti.data.as_slice());

        data_reader.read_into_buf().unwrap();

        // parse SendTable's
        let mut send_tables = Vec::new();
        while data_reader.buf_len() > 0 {
            let net_result = NetMessage::parse_from_bufredux_reader(&mut data_reader);

            let mut message = None;
            if let Ok((msg, _warn)) = net_result {
                message = Some(msg);
            }

            if message.is_none() {
                return Err("couldn't parse SendTable message")
            }

            let mut is_end = false;
            if let NetMessage::SendTable(st) = message.unwrap() {
                if st.is_end.is_some() {
                    is_end = st.is_end.unwrap() > 0;
                }
                send_tables.push(st);
            } else {
                return Err("expected SendTable messages only")
            }

            if is_end { // breakout to read class descriptions
                break;
            }
        }

        // parse class descriptions
        let mut class_descriptions = Vec::new();
        {
            let class_count = parse_u16(&mut data_reader)?;

            while data_reader.buf_len() > 0 {
                let class_id = parse_u16(&mut data_reader)?;
                let network_name = parse_cstr(&mut data_reader)?;
                let table_name = parse_cstr(&mut data_reader)?;

                class_descriptions.push(DataTablesClassDescription { class_id, network_name, table_name });
            }

            if class_descriptions.len() != class_count as usize {
                return Err("too little or too many class descriptions were received")
            }
        }

        Ok(Self { send_tables, class_descriptions })
    }
}

pub enum CommandIndex {
    SignOn(PacketIndex),
    Packet(PacketIndex),
    SyncTick,
    DataTables(DataTablesIndex),
    Stop,
}

impl CommandIndex {
    pub fn from_u8_and_reader(num: u8, mut reader: impl Read) -> Result<Self, EventData> {
        match num {
            command_id::SIGN_ON => Ok(
                Self::SignOn(match PacketIndex::from_readable(&mut reader) {
                    Ok(soi) => soi,
                    Err(_e) => return Err(event!{"SignOnIndex parse error"})
                })
            ),
            command_id::PACKET => Ok(
                Self::Packet(match PacketIndex::from_readable(&mut reader) {
                    Ok(pi) => pi,
                    Err(_e) => return Err(event!{"PacketIndex parse error"})
                })
            ),
            command_id::SYNC_TICK => Ok(Self::SyncTick),
            command_id::DATA_TABLES => Ok(
                Self::DataTables(match DataTablesIndex::from_readable(&mut reader) {
                    Ok(dti) => dti,
                    Err(_e) => return Err(event!{"DataTablesIndex parse error"})
                })
            ),
            command_id::STOP => Ok(Self::Stop),
            _ => {
                let mut ev = event!{""};
                ev.details = format!{"unsupported command number: {}", num};
                Err(ev)
            }
        }
    }
}

mod command_id {
    pub const      SIGN_ON: u8 = 1;
    pub const       PACKET: u8 = 2;
    pub const    SYNC_TICK: u8 = 3;
    //pub const  CONSOLE_CMD: u8 = 4;
    //pub const     USER_CMD: u8 = 5;
    pub const  DATA_TABLES: u8 = 6;
    pub const         STOP: u8 = 7;
    //pub const STRING_TABLE: u8 = 8;
}