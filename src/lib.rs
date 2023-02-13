pub mod demo_file;
pub mod engine_types;
pub mod event_data;

mod parse_tools;
mod bitbuf;

pub mod protobuf_message;

pub extern crate source_demo_tool_impl_proc_macros;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::demo_file::{DemoFile, frame::Command, packet::{MessageParseReturn, netmessage::NetMessage}};

    use crate::demo_file::packet::netmessage::PacketEntityDataParse;
    #[test]
    fn test_packets() -> Result<(), String> {
        let df = match DemoFile::open(&PathBuf::from("assets/test_demos/full_gotv.dem")) {
            Ok(x) => x,
            Err(s) => return Err(s.into())
        };

        let si = df.get_server_info().unwrap();

        for f in &df.frames {
            if let Command::Packet(pd) = &f.command {
                for nmsg_ret in &pd.network_messages {
                    if let NetMessage::PacketEntity(ped) = nmsg_ret.message.as_ref().unwrap() {
                        println!(
                            "{:#?}",
                            ped.parse(si.max_classes.unwrap())
                        );
                        return Ok(())
                    }
                }
            }
        }

        Ok(())
    }
}

/*
#[cfg(test)]
mod tests {
    use std::{ fs::File, io::Write, collections::BTreeMap, };

    use crate::demo_file::{
        frame::Command,
        packet::netmessage::NetMessage,
    };
    use super::*;

    fn full_file_open_routine(filepath: &str, tag: &str) -> Result<(), String> {
        let demo_file = match demo_file::DemoFile::open(filepath) {
            Ok(df) => df,
            Err(e) => return Err(format!("couldn't open test file: {e}"))
        };

        let output_first_100_frames = format!("{:#?}", &demo_file.frames[0..100]);
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-first_100_frames.txt").unwrap();
        out_file.write_all(output_first_100_frames.as_bytes()).unwrap();

        let out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-summary.txt").unwrap();
        summarize_demo_file(&demo_file, out_file);

        let header = format!("{:#?}", &demo_file.header);
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-header.txt").unwrap();
        out_file.write_all(header.as_bytes()).unwrap();

        let mut user_messages = Vec::new();
        let mut game_events = Vec::new();

        let mut first_1000_entity_data = Vec::new();
        for frame in &demo_file.frames {
            if let Command::Packet(packet) = &frame.command {
                for netmsg in &packet.network_messages {
                    let msg = netmsg.message.as_ref().unwrap();
                    match msg {
                        NetMessage::PacketEntity(pe) => {
                            if first_1000_entity_data.len() == 1000 {
                                continue
                            }
                            first_1000_entity_data.push(pe);
                        },
                        NetMessage::UserMessage(um) => {
                            user_messages.push(um);
                        },
                        NetMessage::GameEvent(ge) => {
                            game_events.push(ge);
                        },
                        _ => {}
                    }
                }
            }
        }

        // raw entity data
        let mut output_raw_entity_data = Vec::new();
        for ed in first_1000_entity_data {
            output_raw_entity_data.append(&mut ed.entity_data.clone().unwrap());
            output_raw_entity_data.append(&mut [0xDE, 0xAD, 0xFF, 0xDE, 0xAD, 0xFF].to_vec());
        }
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-first_1000_entity_data.raw").unwrap();
        out_file.write_all(output_raw_entity_data.as_slice()).unwrap();

        // sign on frames
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-signon_frames.txt").unwrap();
        out_file.write_all(format!{"{:#?}", &demo_file.sign_on_frames}.as_bytes()).unwrap();

        // user messages
        let mut output_user_messages = Vec::new();
        for um in user_messages {
            output_user_messages.append(&mut um.msg_type.unwrap().to_le_bytes().to_vec());
            output_user_messages.append(&mut um.msg_data.as_ref().unwrap().clone());
            output_user_messages.append(&mut [0xDE, 0xAD, 0xFF, 0xDE, 0xAD, 0xFF].to_vec());
        }
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-user_messages.raw").unwrap();
        out_file.write_all(output_user_messages.as_slice()).unwrap();

        // data tables
        let output_data_tables = format!("{:#?}", demo_file.get_data_tables());
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-data_tables.txt").unwrap();
        out_file.write_all(output_data_tables.as_bytes()).unwrap();

        // game events
        let output_game_events = format!("{:#?}", game_events);
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-game_events.txt").unwrap();
        out_file.write_all(output_game_events.as_bytes()).unwrap();

        // server info
        let output_server_info = format!("{:#?}", demo_file.get_server_info().unwrap());
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-server_info.txt").unwrap();
        out_file.write_all(output_server_info.as_bytes()).unwrap();

        // sign on data summary
        {
            let mut command_counter = BTreeMap::new();
            let mut message_counter = BTreeMap::new();
            let mut class_info = None;
            let mut server_info = None;
            let mut game_event_list = None;
            let mut set_con_var = None;
            let mut set_view = None;
            let mut sign_on_states = Vec::new();
            let mut tick = None;
            let mut voice_init = None;
            let mut create_string_table_data_messages = Vec::new();
            for f in &demo_file.sign_on_frames {
                match &f.command {
                    Command::DataTables(_dt) => {
                        command_counter.entry("DataTables").and_modify(|e| {*e+=1}).or_insert(1);
                    },
                    Command::Packet(_pd) => {
                        command_counter.entry("Packet").and_modify(|e| {*e+=1}).or_insert(1);
                    },
                    Command::SignOn(pd) => {
                        command_counter.entry("SignOn").and_modify(|e| {*e+=1}).or_insert(1);

                        for msg in &pd.network_messages {
                            if let Some(nmsg) = &msg.message {
                                message_counter.entry(nmsg.to_string()).and_modify(|e| {*e+=1}).or_insert(1);
                                match &nmsg {
                                    NetMessage::ClassInfo(ci) => class_info = Some(ci),
                                    NetMessage::ServerInfo(si) => server_info = Some(si),
                                    NetMessage::GameEventList(gel) => game_event_list = Some(gel),
                                    NetMessage::SetConVar(scv) => set_con_var = Some(scv),
                                    NetMessage::SetView(sv) => set_view = Some(sv),
                                    NetMessage::SignOnState(sos) => sign_on_states.push(sos),
                                    NetMessage::Tick(td) => tick = Some(td),
                                    NetMessage::VoiceInit(vid) => voice_init = Some(vid),
                                    NetMessage::CreateStringTable(cstd) => create_string_table_data_messages.push(cstd),
                                    _ => {}
                                }
                                
                            }
                        }
                    },
                    Command::Stop => {
                        command_counter.entry("Stop").and_modify(|e| {*e+=1}).or_insert(1);
                    },
                    Command::SyncTick => {
                        command_counter.entry("SyncTick").and_modify(|e| {*e+=1}).or_insert(1);
                    }
                }
            }

            let output_sod_summary = format! {
                "\
                command_counter \
                    {:#?}\n\
                message_counter \
                    {:#?}\n\
                class_info \
                    {:#?}\n\
                server_info \
                    {:#?}\n\
                set_con_var \
                    {:#?}\n\
                set_view \
                    {:#?}\n\
                sign_on_state \
                    {:#?}\n\
                tick \
                    {:#?}\n\
                voice_init \
                    {:#?}\n\
                "
                , command_counter, message_counter, class_info, server_info, set_con_var, set_view,
                sign_on_states, tick, voice_init
            };
            let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-sign_on_data_summary.txt").unwrap();
            out_file.write_all(output_sod_summary.as_bytes()).unwrap();

            // game event list
            let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-game_event_list.txt").unwrap();
            out_file.write_all(format!{"{:#?}", game_event_list}.as_bytes()).unwrap();

            // create string table data
            let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-create_string_table.txt").unwrap();
            out_file.write_all(format!{"{:#?}", create_string_table_data_messages}.as_bytes()).unwrap();
        }

        // dump full game events
        let full_game_events = demo_file.get_full_game_events();
        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-full_game_events.txt").unwrap();
            out_file.write_all(format!{"{:#?}", &full_game_events}.as_bytes()).unwrap();

        let mut game_event_counter = BTreeMap::new();
        let mut game_event_id_name_map = BTreeMap::new();
        for ev in &full_game_events {
            game_event_counter.entry(&ev.event_id).and_modify(|e|{*e+=1}).or_insert(1);
            game_event_id_name_map.insert(&ev.event_id, &ev.event_name);
        }

        let mut out_file = File::create("test_output/file_full_open-".to_owned() + tag + "-game_events_summary.txt").unwrap();
        out_file.write_all(format!("{:>6}{:>42}{:>10}\n", "id", "name", "count").as_bytes()).unwrap();
        for id in game_event_id_name_map {
            out_file.write_all(format!("{:>6}{:>42}{:>10}\n", id.0, id.1, game_event_counter[id.0]).as_bytes()).unwrap();
        }

        Ok(())
    }

    #[test]
    fn file_full_gotv_open1_full_gotv() -> Result<(), String> {
        full_file_open_routine("assets/test_demos/full_gotv.dem", "full_gotv")
    }

    #[test]
    fn file_full_gotv_open2_glitched() -> Result<(), String> {
        full_file_open_routine("assets/test_demos/glitched_texture_r14_acor.dem", "glitched")
    }

    #[test]
    fn file_full_gotv_open3_full_gotv() -> Result<(), String> {
        full_file_open_routine("assets/test_demos/gamerlegion-vs-masonic-m1-vertigo.dem", "full_gotv3")
    }

    #[test]
    fn file_full_gotv_open4_full_gotv() -> Result<(), String> {
        full_file_open_routine("assets/test_demos/gamerlegion-vs-masonic-m3-ancient.dem", "full_gotv4")
    }

    fn summarize_demo_file(demo_file: &demo_file::DemoFile, mut out_file: File) {
        let mut unknown_message_counter = BTreeMap::new();
        let mut unknown_field_counter = BTreeMap::new();
        let mut missing_field_counter = BTreeMap::new();
        let mut invalid_or_corrupt_events = BTreeMap::new();
        let mut frame_command_counter = BTreeMap::new();
        let mut frame_player_slot_counter = BTreeMap::new();
        let mut message_counter = BTreeMap::new();
        let mut first_10_sub_warnings = Vec::new();
        let mut user_message_counter = BTreeMap::new();

        for frame in &demo_file.frames {
            frame_command_counter.entry(frame.command.as_u8())
                .and_modify(|e| {*e += 1 })
                .or_insert(1);

            frame_player_slot_counter.entry(&frame.player_slot)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);

            if let Command::Packet(p) = &frame.command {
                for netmsg in &p.network_messages {
                    if let Some(msg) = &netmsg.message {
                        message_counter.entry(msg.to_string())
                            .and_modify(|e| { *e += 1 })
                            .or_insert(1);
                        if let NetMessage::UserMessage(umd) = &netmsg.message.as_ref().unwrap() {
                            user_message_counter.entry(umd.msg_type.unwrap())
                            .and_modify(|e|{*e+=1}).or_insert(1);
                        }
                    }

                    if let Some(msg_warns) = &netmsg.warnings {
                        let message_name = netmsg.message.as_ref().unwrap().to_string();
                        for field in &msg_warns.missing_fields {
                            missing_field_counter.entry((message_name.clone(), field.0, field.1))
                                .and_modify(|e| { *e += 1 })
                                .or_insert(1);
                        }
                        for field in &msg_warns.unknown_fields {
                            unknown_field_counter.entry((message_name.clone(), field.field_number))
                                .and_modify(|(_, counter)| { *counter += 1})
                                .or_insert((field.clone(), 1));
                        }
                        for field in &msg_warns.sub_warnings {
                            if first_10_sub_warnings.len() < 10 {
                                if !(field.missing_fields.is_empty() &&
                                    field.unknown_fields.is_empty() &&
                                    field.sub_warnings.is_empty()) {
                                    first_10_sub_warnings.push((message_name.clone(), field));
                                }
                            }
                        }
                    }

                    if let Some(msg) = &netmsg.err {
                        type ErrT = demo_file::packet::ParseMessageErr;
                        match msg {
                            ErrT::InvalidOrCorrupt(ev) => { invalid_or_corrupt_events.entry(ev)
                                .and_modify(|e| {*e += 1 })
                                .or_insert(1); },
                            ErrT::UnknownCommand(n) => {
                                unknown_message_counter.entry(n)
                                    .and_modify(|e| { *e += 1 })
                                    .or_insert(1);
                            }
                        }
                    }
                }
            }
        }

        out_file.write_all(b"Unknown Messages -- msg_num: Count\n").unwrap();
        out_file.write_all(format!("{:#?}", unknown_message_counter).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"Missing Fields -- (message_name, field_num, field_name): Count\n").unwrap();
        out_file.write_all(format!("{:#?}", missing_field_counter).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"Unknown Fields -- (message_name, field_num): Count\n").unwrap();
        out_file.write_all(format!("{:#?}", unknown_field_counter).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"Invalid or corrupt frame -- Event: Count\n").unwrap();
        out_file.write_all(format!("{:#?}", invalid_or_corrupt_events).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"Frame Commands -- Command: Count\n").unwrap();
        out_file.write_all(format!("{:#?}", frame_command_counter).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"Player Slot -- player_slot: Count\n").unwrap();
        out_file.write_all(format!("{:#?}", frame_player_slot_counter).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"Messages -- message_name: Count\n").unwrap();
        out_file.write_all(format!("{:#?}", message_counter).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"First 10 sub-warnings\n").unwrap();
        out_file.write_all(format!("{:#?}", first_10_sub_warnings).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"UserMessages -- msg_type: Count\n").unwrap();
        out_file.write_all(format!("{:#?}", user_message_counter).as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();

        out_file.write_all(b"UserMessage warnings & errors\n").unwrap();
        let user_messages = demo_file.get_user_messages();
        let mut user_messages_warnings = Vec::new();
        let mut user_messages_errors = Vec::new();
        let mut first_10_sub_warnings = Vec::new();
        for um in &user_messages {
            if let Some(warns) = &um.warnings {
                for sub_warn in &warns.sub_warnings {
                    if !(sub_warn.missing_fields.is_empty() && sub_warn.unknown_fields.is_empty()) {
                        first_10_sub_warnings.push(sub_warn);
                    }
                }

                if !(warns.missing_fields.is_empty() && warns.unknown_fields.is_empty()) {
                    user_messages_warnings.push(warns);
                }

            }

            if let Some(err) = &um.err {
                user_messages_errors.push(err);
            }
        }
        out_file.write_all(format!("\
        Warnings \
          {:#?}\n\
        Errors \
          {:#?}\n\
        First 10 sub warnings \
          {:#?}\
        ", user_messages_warnings, user_messages_errors, first_10_sub_warnings).as_bytes()).unwrap();
    }
}
*/