use source_demo_tool_impl_proc_macros::declare_protobuf_messages;

declare_protobuf_messages!(Net, {
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
                is_end: VarInt = 1,
        net_table_name: String = 2,
         needs_decoder: VarInt = 3,
        repeated SendProp:  Proto = 4 {
            sendprop_type:  VarInt = 1,
                 var_name:  String = 2,
                    flags:  VarInt = 3,
                 priority:  VarInt = 4,
                  dt_name:  String = 5,
             num_elements:  VarInt = 6,
                low_value: Fixed32 = 7,
               high_value: Fixed32 = 8,
                 num_bits:  VarInt = 9,
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
    AvatarData = 100 {
        account_id: VarInt = 1,
         rgb_bytes: Length = 2,
    },
});