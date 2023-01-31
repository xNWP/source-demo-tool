use source_demo_tool_impl_proc_macros::declare_protobuf_messages;

declare_protobuf_messages!(Net, {
    Tick = 4 {
        optional                     current_tick: VarInt = 1,
        optional            host_computation_time: VarInt = 4,
        optional         host_computation_std_dev: VarInt = 5,
        optional    host_frame_start_time_std_dev: VarInt = 6,
    },
    StringCmd = 5 {
        optional command: String = 1,
    },
    SetConVar = 6 {
        repeated ConVars: Proto  = 1 {
            optional name: String = 1, 
        },
    },
    SignOnState = 7 {
        optional       signon_state: VarInt = 1,
        optional        spawn_count: VarInt = 2,
        optional num_server_players: VarInt = 3,
    },
    ServerInfo = 8 {
        optional         protocol:  VarInt =  1,
        optional  server_restarts:  VarInt =  2,
        optional     is_dedicated:  VarInt =  3,
        optional          is_hltv:  VarInt =  5,
        optional               os:  VarInt =  7,
        optional          map_crc: Fixed32 =  8,
        optional       client_crc: Fixed32 =  9,
        optional string_table_crc: Fixed32 = 10,
        optional      max_clients:  VarInt = 11,
        optional      max_classes:  VarInt = 12,
        optional      player_slot:  VarInt = 13,
        optional    tick_interval: Float32 = 14,
        optional         game_dir:  String = 15,
        optional         map_name:  String = 16,
        optional   map_group_name:  String = 17,
        optional         sky_name:  String = 18,
        optional        host_name:  String = 19,
        optional        unknown21:  VarInt = 21,
        optional       ugc_map_id:  VarInt = 22,
        optional        unknown23:  VarInt = 23,
    },
    SendTable = 9 {
        optional         is_end: VarInt = 1,
        optional net_table_name: String = 2,
        optional  needs_decoder: VarInt = 3,
        repeated       SendProp:  Proto = 4 {
            optional sendprop_type:  VarInt = 1,
            optional      var_name:  String = 2,
            optional         flags:  VarInt = 3,
            optional      priority:  VarInt = 4,
            optional       dt_name:  String = 5,
            optional  num_elements:  VarInt = 6,
            optional     low_value: Float32 = 7,
            optional    high_value: Float32 = 8,
            optional      num_bits:  VarInt = 9,
        }
    },
    ClassInfo = 10 {
        optional is_create_on_client: VarInt = 1,
    },
    CreateStringTable = 12 {
        optional                    name: String = 1,
        optional             max_entries: VarInt = 2,
        optional             num_entries: VarInt = 3,
        optional is_user_data_fixed_size: VarInt = 4,
        optional          user_data_size: VarInt = 5,
        optional     user_data_size_bits: VarInt = 6,
        optional                   flags: VarInt = 7,
        optional             string_data: Length = 8,
    },
    UpdateStringTable = 13 {
        optional               table_id: VarInt = 1,
        optional    num_changed_entries: VarInt = 2,
        optional            string_data: Length = 3,
        
    },
    VoiceInit = 14 {
        optional quality: VarInt = 1,
        optional   codec: String = 2,
        optional version: VarInt = 3,
    },
    Sounds = 17 {
        optional reliable_sound: VarInt = 1,
        repeated SoundsInner: Proto = 2 {
            optional            origin_x:  VarInt = 1,
            optional            origin_y:  VarInt = 2,
            optional            origin_z:  VarInt = 3,
            optional              volume:  VarInt = 4,
            optional     sequence_number:  VarInt = 6,
            optional        entity_index:  VarInt = 7,
            optional             channel:  VarInt = 8,
            optional               pitch:  VarInt = 9,
            optional               flags:  VarInt = 10,
            optional           sound_num:  VarInt = 11,
            optional    sound_num_handle: Fixed32 = 12,
            optional         random_seed:  VarInt = 14,
            optional         sound_level:  VarInt = 15,
        },
    },
    SetView = 18 {
        optional entity_index: VarInt = 1,
    },
    UserMessage = 23 {
        optional msg_type: VarInt = 1,
        optional msg_data: Length = 2,
    },
    GameEvent = 25 {
        optional event_id: VarInt = 2,
        optional repeated GameEventKeys: Proto = 3 {
            optional   val_type:  VarInt = 1,
            optional val_string:  String = 2,
            optional  val_float: Float32 = 3,
            optional   val_long:  VarInt = 4,
            optional  val_short:  VarInt = 5,
            optional   val_byte:  VarInt = 6,
            optional   val_bool:  VarInt = 7,
        }
    },
    PacketEntity = 26 {
        optional        max_entries: VarInt = 1,
        optional    updated_entries: VarInt = 2,
        optional           is_delta: VarInt = 3,
        optional    update_baseline: VarInt = 4,
        optional           baseline: VarInt = 5,
        optional         delta_from: VarInt = 6,
        optional        entity_data: Length = 7
    },
    TempEntities = 27 {
        optional reliable: VarInt = 1,
        optional num_entries: VarInt = 2,
        optional entity_data: Length = 3,
    },
    Prefetch = 28 {
        optional sound_index: VarInt = 1,
    },
    GameEventList = 30 {
        repeated Descriptors: Proto = 1 {
            optional          event_id: VarInt = 1,
            optional              name: String = 2,
            repeated    DescriptorKeys:  Proto = 3 {
                optional key_type: VarInt = 1,
                optional key_name: String = 2,
            },
        },
    },
});