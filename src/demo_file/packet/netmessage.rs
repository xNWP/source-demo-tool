use std::collections::BTreeMap;

use ilog::IntLog;
use source_demo_tool_impl_proc_macros::declare_protobuf_messages;

use crate::bitbuf::{BitBuf, self};

declare_protobuf_messages!(Net, {
    Nop = 0 {

    },
    Tick = 4 {
                         current_tick: VarInt = 1,
                host_computation_time: VarInt = 4,
             host_computation_std_dev: VarInt = 5,
        host_frame_start_time_std_dev: VarInt = 6,
    },
    StringCmd = 5 {
        command: String = 1,
    },
    SetConVar = 6 {
        ConVars: Proto = 1 {
            repeated ConvarsInner: Proto = 1 {
                optional name: String = 1,
                        value: String = 2,
                optional   v3: VarInt = 3,
            },
        },
    },
    SignOnState = 7 {
              signon_state: VarInt = 1,
               spawn_count: VarInt = 2,
        num_server_players: VarInt = 3,
               optional s5: String = 5,
    },
    ServerInfo = 8 {
                protocol:  VarInt =  1,
         server_restarts:  VarInt =  2,
            is_dedicated:  VarInt =  3,
                 is_hltv:  VarInt =  5,
                      os:  VarInt =  7,
                 map_crc: Fixed32 =  8,
              client_crc: Fixed32 =  9,
        string_table_crc: Fixed32 = 10,
             max_clients:  VarInt = 11,
             max_classes:  VarInt = 12,
             player_slot:  VarInt = 13,
           tick_interval: Float32 = 14,
                game_dir:  String = 15,
                map_name:  String = 16,
          map_group_name:  String = 17,
                sky_name:  String = 18,
               host_name:  String = 19,
               unknown21:  VarInt = 21,
              ugc_map_id:  VarInt = 22,
        optional     v23:  VarInt = 23,
    },
    SendTable = 9 {
        optional            is_end: VarInt = 1,
        optional    net_table_name: String = 2,
        optional     needs_decoder: VarInt = 3,
        optional repeated SendProp:  Proto = 4 {
                sendprop_type:  VarInt = 1,
                     var_name:  String = 2,
                        flags:  VarInt = 3,
                     priority:  VarInt = 4,
        optional      dt_name:  String = 5,
        optional num_elements:  VarInt = 6,
        optional    low_value: Fixed32 = 7,
        optional   high_value: Fixed32 = 8,
        optional     num_bits:  VarInt = 9,
        }
    },
    ClassInfo = 10 {
        is_create_on_client: VarInt = 1,
    },
    CreateStringTable = 12 {
                           name: String = 1,
                    max_entries: VarInt = 2,
                    num_entries: VarInt = 3,
        is_user_data_fixed_size: VarInt = 4,
                 user_data_size: VarInt = 5,
            user_data_size_bits: VarInt = 6,
                          flags: VarInt = 7,
                    string_data: Length = 8,
    },
    UpdateStringTable = 13 {
                   table_id: VarInt = 1,
        num_changed_entries: VarInt = 2,
                string_data: Length = 3,

    },
    VoiceInit = 14 {
        quality: VarInt = 1,
          codec: String = 2,
        version: VarInt = 3,
    },
    Print = 16 {
        text: String = 1,
    },
    Sounds = 17 {
        reliable_sound: VarInt = 1,
        repeated SoundsInner: Proto = 2 {
            optional         origin_x:  VarInt =  1,
            optional         origin_y:  VarInt =  2,
            optional         origin_z:  VarInt =  3,
            optional           volume:  VarInt =  4,
            optional  sequence_number:  VarInt =  6,
            optional     entity_index:  VarInt =  7,
            optional          channel:  VarInt =  8,
            optional            pitch:  VarInt =  9,
            optional            flags:  VarInt = 10,
            optional        sound_num:  VarInt = 11,
            optional sound_num_handle: Fixed32 = 12,
            optional      random_seed:  VarInt = 14,
            optional      sound_level:  VarInt = 15,
            optional       is_ambient:  VarInt = 17,
        },
    },
    SetView = 18 {
        entity_index: VarInt = 1,
    },
    BspDecal = 21 {
        BspDecalPos: Proto = 1 {
            _x: Float32 = 1,
            _y: Float32 = 2,
            _z: Float32 = 3,
        },
          texture_index: VarInt = 2,
           entity_index: VarInt = 3,
            model_index: VarInt = 4,
        is_low_priority: VarInt = 5,
    },
    UserMessage = 23 {
        msg_type: VarInt = 1,
        msg_data: Length = 2,
    },
    GameEvent = 25 {
        event_id: VarInt = 2,
        optional repeated GameEventKeys: Proto = 3 {
                       val_type:  VarInt = 1,
            optional val_string:  String = 2,
            optional  val_float: Float32 = 3,
            optional   val_long:  VarInt = 4,
            optional  val_short:  VarInt = 5,
            optional   val_byte:  VarInt = 6,
            optional   val_bool:  VarInt = 7,
        }
    },
    PacketEntity = 26 {
                max_entries: VarInt = 1,
            updated_entries: VarInt = 2,
                   is_delta: VarInt = 3,
            update_baseline: VarInt = 4,
                   baseline: VarInt = 5,
        optional delta_from: VarInt = 6,
                entity_data: Length = 7
    },
    TempEntities = 27 {
        optional reliable: VarInt = 1,
              num_entries: VarInt = 2,
              entity_data: Length = 3,
    },
    Prefetch = 28 {
        sound_index: VarInt = 1,
    },
    GameEventList = 30 {
        repeated Descriptors: Proto = 1 {
            event_id: VarInt = 1,
                name: String = 2,
            optional repeated DescriptorKeys: Proto = 3 {
                key_type: VarInt = 1,
                key_name: String = 2,
            },
        },
    },
    CmdKeyValues = 34 {
        key_values: Length = 1,
    },
    AvatarData = 100 {
        account_id: VarInt = 1,
         rgb_bytes: Length = 2,
    },
});

#[derive(Debug)]
pub struct ParsedPacketData {
    data: Vec<ParsedPacketDataInner>,
}

#[derive(Debug)]
pub struct ParsedPacketDataInner {
    new_entity_idx: i32,
    //update_flags: UpdateFlags,
    update_type: UpdateType,
    field_indices: Vec<(i32, usize)>,
}

pub trait PacketEntityDataParse {
    fn parse(&self, max_server_classes: u64) -> ParsedPacketData;
}

impl PacketEntityDataParse for PacketEntityData {
    fn parse(&self, max_server_classes: u64) -> ParsedPacketData {

        let server_class_bits = max_server_classes.log2() + 1;

        let mut data = Vec::new();

        let bit_buf = BitBuf::new(
            self.entity_data
                .as_ref().unwrap().clone()
        );

        let mut ent_ri = EntityReadInfo::new(bit_buf, &self);

        let mut update_type = ent_ri.update_type;
        let old_entity_idx = ent_ri.old_ent_idx;
        let old_entity_idx = ent_ri.get_next_old_entity(old_entity_idx);

        let mut new_entity_idx: i32 = -1;
        let mut new_entity_idx_base: i32 = -1;

        const DEBUG_COUNTS: usize = 3;
        let mut debug_it = 1;
        loop {
            if update_type == UpdateType::Finished {
                break;
            }

           // let mut update_flags = UpdateFlags::new(0);
            ent_ri.header_count -= 1;

            if ent_ri.header_count >= 0 {
                parse_delta_header(&mut ent_ri);
            } else {
                todo!("Not entity");
            }
            let mut field_indices = Vec::new();

            update_type = UpdateType::PreserveEnt;
            while update_type == UpdateType::PreserveEnt {
                update_type = {
                    if ent_ri.header_count < 0 || ent_ri.new_ent_idx > old_entity_idx {
                        todo!("not done")
                    } else {
                        if ent_ri.update_flags & update_flags::ENTER_PVS != 0 {
                            UpdateType::EnterPVS
                        } else if ent_ri.update_flags & update_flags::LEAVE_PVS != 0 {
                            UpdateType::LeavePVS
                        } else {
                            UpdateType::DeltaEnt
                        }
                    }
                };

                println!("{:?}", update_type);

                match &update_type {
                    UpdateType::EnterPVS => {
                        let class_idx = ent_ri.buf.read_ubit_long(server_class_bits.try_into().unwrap());
                        println!(" Class Idx: {}", class_idx);
                        let serial_num = ent_ri.buf.read_ubit_long(10);
                        println!("Serial Num: {}", serial_num);

                        ent_ri.old_ent_idx = old_entity_idx;
                        copy_new_entity(&mut ent_ri, class_idx, serial_num);
                    },
                    UpdateType::LeavePVS => {
                        assert!(!(self.is_delta.unwrap() > 0), "Must be delta update.");
                    },
                    UpdateType::DeltaEnt => {

                    },
                    ty => todo!("UpdateType not supported: {:?}", ty)
                }
            }

            data.push(ParsedPacketDataInner {
                new_entity_idx,
                //update_flags,
                update_type,
                field_indices,
            });

            if debug_it == DEBUG_COUNTS {
                break;
            } else {
                debug_it += 1;
            }
        }

        ParsedPacketData {
            data
        }
    }
}

fn get_prop_idx(bit_buf: &mut BitBuf, last_prop_idx: &mut i32, using_new_scheme: bool) -> i32 {
    if using_new_scheme {
        if bit_buf.read_one_bit() {
            *last_prop_idx += 1;
            return *last_prop_idx
        } else {
            // panic!()
        }
    }

    if using_new_scheme {
        if bit_buf.read_one_bit() {
            return bit_buf.read_ubit_long(3).try_into().unwrap()
        } else {
            //panic!()
        }
    }

    let mut rval = bit_buf.read_ubit_long(7);
    rval = match rval & (32 | 64) {
        32 => (rval & 31) | (bit_buf.read_ubit_long(2) << 5),
        64 => (rval & 31) | (bit_buf.read_ubit_long(4) << 5),
        96 => (rval & 31) | (bit_buf.read_ubit_long(7) << 5),
        _ => rval
    };

    rval.try_into().unwrap()
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum UpdateType {
    EnterPVS,
    LeavePVS,
    DeltaEnt,
    PreserveEnt,
    Finished,
    Failed,
}

impl Into<u8> for UpdateType {
    fn into(self) -> u8 {
        match self {
            UpdateType::EnterPVS    => 0,
            UpdateType::LeavePVS    => 1,
            UpdateType::DeltaEnt    => 2,
            UpdateType::PreserveEnt => 3,
            UpdateType::Finished    => 4,
            UpdateType::Failed      => 5,
        }
    }
}

mod update_flags {
    pub const LEAVE_PVS: i32 = 1;
    pub const    DELETE: i32 = 2;
    pub const ENTER_PVS: i32 = 4;
}

struct EntityReadInfo {
    buf: BitBuf,
    update_type: UpdateType,
    header_count: i32,
    new_ent_idx: i32,
    old_ent_idx: i32,
    header_base: i32,
    update_flags: i32,
    from_frame: Option<()>,
}

impl EntityReadInfo {
    pub fn new(buf: BitBuf, msg: &PacketEntityData) -> Self {
        Self {
            buf,
            update_type: UpdateType::PreserveEnt,
            header_count: msg.updated_entries.unwrap().try_into().unwrap(),
            new_ent_idx: -1,
            old_ent_idx: -1,
            header_base: -1,
            update_flags: 0,
            from_frame: None,
        }
    }

    pub fn get_next_old_entity(&self, _start_entity: i32) -> i32 {
        if self.from_frame.is_some() {
            todo!("delta frames not yet implemented.")
        } else {
            ENTITY_SENTINEL
        }
    }
}

const ENTITY_SENTINEL: i32 = 9999;

fn parse_delta_header(ri: &mut EntityReadInfo) {
    ri.update_flags = 0;

    let new_idx: i32 = ri.buf.read_ubit_var().try_into().unwrap();
    ri.new_ent_idx = ri.header_base + 1 + new_idx;
    ri.header_base = ri.new_ent_idx;

    if !ri.buf.read_one_bit() {
        if ri.buf.read_one_bit() {
            ri.update_flags |= update_flags::ENTER_PVS;
        }
    } else {
        ri.update_flags |= update_flags::LEAVE_PVS;

        if ri.buf.read_one_bit() {
            ri.update_flags |= update_flags::DELETE;
        }
    }
}

fn copy_new_entity(ri: &mut EntityReadInfo, _class_idx: u32, _serial_num: u32) {
    if ri.new_ent_idx < 0 || ri.new_ent_idx >= (1 << 11) {
        panic!()
    }

    // get entity from state
    // if entity exists and serial number differs, delete
    // the entity and reinit the object

    read_field_lists(&mut ri.buf);

    let start_bit = ri.buf.get_bits_read();
}

fn read_field_lists(buf: &mut BitBuf) {
    let indices = read_field_paths(buf);
    println!("indices: {:?}", indices);

    let mut offsets = Vec::new();

    let start_bit = buf.get_bits_read();
    for f in indices {
        let data_offset = buf.get_bits_read();

        offsets.push(data_offset - start_bit);

    }
}

fn read_field_paths(buf: &mut BitBuf) -> Vec<(i32, i32)> {
    let mut reader = DeltaBitsReader::new(buf);

    let mut values = Vec::new();
    loop {
        let value = reader.read_next_prop_idx(buf);

        if value.0 == -1 {
            break
        }

        values.push(value);
    }

    values
}

struct DeltaBitsReader {
    using_new_scheme: bool,
    last_prop_idx: i32,
}

impl DeltaBitsReader {
    pub fn new(buf: &mut BitBuf) -> Self {
        let using_new_scheme = buf.read_one_bit();

        Self {
            using_new_scheme,
            last_prop_idx: -1,
        }
    }

    pub fn read_next_prop_idx(&mut self, buf: &mut BitBuf) -> (i32, i32) {
        if self.using_new_scheme {
            println!("A here");
            if buf.read_one_bit() {
                println!("B here");
                self.last_prop_idx += 1;
                return (self.last_prop_idx, 1)
            }
        }

        let start_bit: i32 = buf.get_bits_read().try_into().unwrap();
        let idx = self.read_prop_index(buf);
        let end_bit: i32 = buf.get_bits_read().try_into().unwrap();
        let bits_read = end_bit - start_bit;

        if idx == (1 << 12) - 1 {
            println!("C here, idx: {}", idx);
            return (-1, bits_read)
        }

        let mut prop = (1 + idx).try_into().unwrap();
        prop += self.last_prop_idx;
        self.last_prop_idx = prop;
        (prop, bits_read)
    }

    fn read_prop_index(&mut self, buf: &mut BitBuf) -> u32 {
        if self.using_new_scheme {
            if buf.read_one_bit() {
                return buf.read_ubit_long(3)
            }
        }

        let rval = buf.read_ubit_long(7);
        match rval & (32 | 64) {
            32 => (rval & 31) | (buf.read_ubit_long(2) << 5),
            64 => (rval & 31) | (buf.read_ubit_long(4) << 5),
            96 => (rval & 31) | (buf.read_ubit_long(7) << 5),
            _ => rval
        }
    }
}