use source_demo_tool_impl_proc_macros::declare_protobuf_messages;

declare_protobuf_messages!(User, {
    SayText = 5 {
        optional       entity_idx: VarInt = 1,
                             text: String = 2,
        optional          is_chat: VarInt = 3,
        optional is_text_all_chat: VarInt = 4,
    },
    SayText2 = 6 {
               ent_sidx: VarInt = 1,
                   chat: VarInt = 2,
               msg_name: String = 3,
        repeated params: String = 4,
          text_all_chat: VarInt = 5,
    },
    TextMsg = 7 {
                 msg_dst: VarInt = 1,
        repeated  params: String = 3,
    },
    Shake = 12 {
                command:  VarInt = 1,
        local_amplitude: Float32 = 2,
              frequency: Float32 = 3,
               duration: Float32 = 4,
    },
    Damage = 21 {
        amount: VarInt = 1,
        InflictorWorldPos: Proto = 2 {
            _x: Float32 = 1,
            _y: Float32 = 2,
            _z: Float32 = 3,
        },
        victim_entity_index: VarInt = 3,
    },
    ProcessSpottedEntityUpdate = 25 {
        new_update: VarInt = 1,
        optional repeated SpottedEntityUpdates: Proto = 2 {
                             entity_idx: VarInt = 1,
                               class_id: VarInt = 2,
                               origin_x: VarInt = 3,
                               origin_y: VarInt = 4,
                               origin_z: VarInt = 5,
                                angle_y: VarInt = 6,
                                defuser: VarInt = 7,
            optional player_has_defuser: VarInt = 8,
            optional      player_has_c4: VarInt = 9,
        },
    },
    PlayerStatsUpdate = 36 {
        version: VarInt = 1,
        repeated Stats: Proto = 4 {
              idx: VarInt = 1,
            delta: VarInt = 2,
        },
        user_id: VarInt = 5,
            crc: VarInt = 6,
    },
    WarmupEnded = 38 {

    },
    VoteStart = 46 {
                     team: VarInt = 1,
               entity_idx: VarInt = 2,
                vote_type: VarInt = 3,
           display_string: String = 4,
           details_string: String = 5,
        other_team_string: String = 6,
    },
    VotePass = 47 {
                  team: VarInt = 1,
             vote_type: VarInt = 2,
        display_string: String = 3,
        details_string: String = 4,
    },
    ServerRankRevealAll = 50 {

    },
    P69 = 69 {
         guess_player_idx:  VarInt = 1,
              guess_pos_x: Float32 = 2,
              guess_pos_y: Float32 = 3,
              guess_pos_z: Float32 = 4,
      guess_weapon_action:  String = 5,
          guess_game_time: Float32 = 6,
    },
    P75 = 75 {
        repeated P75_P1: Proto = 1 {
            v1: VarInt = 1,
            v2: VarInt = 2,
            s3: String = 3,
            v4: VarInt = 4,
            P75_P1_P5: Proto = 5 {
                v1:  VarInt = 1,
                f2: Float32 = 2,
                v3:  VarInt = 3,
            },
            repeated P75_P1_P6: Proto = 6 {
                          v2: VarInt = 2,
                          v3: VarInt = 3,
                optional  v4: VarInt = 4,
                optional  v5: VarInt = 5,
                optional  v6: VarInt = 6,
                optional  v7: VarInt = 7,
                optional  v8: VarInt = 8,
                optional  v9: VarInt = 9,
                optional v10: VarInt = 10,
                optional s11: String = 11,
                optional repeated P75_P1_P6_P12: Proto = 12 {
                    v1: VarInt = 1,
                    v2: VarInt = 2,
                    optional f3: Fixed32 = 3,
                    optional f4: Fixed32 = 4,
                    optional f5: Fixed32 = 5,
                },
                optional v14: VarInt = 14,
            },
            v7:  VarInt = 7,
            v8:  VarInt = 8,
        },
    },
});