use source_demo_tool_impl_proc_macros::declare_protobuf_messages;

declare_protobuf_messages!(User, {
    SayText = 5 {
        optional unknown1: VarInt = 1,
                     text: String = 2,
        optional unknown3: VarInt = 3,
        optional unknown4: VarInt = 4,
    },
    SayText2 = 6 {
        optional      ent_sidx: VarInt = 1,
        optional          chat: VarInt = 2,
        optional      msg_name: String = 3,
        optional        params: String = 4,
        optional text_all_chat: VarInt = 5,
    },
    TextMsg = 7 {
        optional msg_dst: VarInt = 1,
        repeated  params: String = 3,
    },
    Shake = 12 {
        optional         command:  VarInt = 1,
        optional local_amplitude: Float32 = 2,
        optional       frequency: Float32 = 3,
        optional        duration: Float32 = 4,
    },
    Damage = 21 {
        optional amount: VarInt = 1,
        optional InflictorWorldPos: Proto = 2 {
            inflictor_world_pos_x: Float32 = 1,
            inflictor_world_pos_y: Float32 = 2,
            inflictor_world_pos_z: Float32 = 3,
        },
        optional victim_entity_index: VarInt = 3,
    },
    ProcessSpottedEntityUpdate = 25 {
        optional                    new_update: VarInt = 1,
        optional repeated SpottedEntityUpdates: Proto = 2 {
            optional         entity_idx: VarInt = 1,
            optional           class_id: VarInt = 2,
            optional           origin_x: VarInt = 3,
            optional           origin_y: VarInt = 4,
            optional           origin_z: VarInt = 5,
            optional            angle_y: VarInt = 6,
            optional            defuser: VarInt = 7,
            optional player_has_defuser: VarInt = 8,
            optional      player_has_c4: VarInt = 9,
        },
    },
    PlayerStatsUpdate = 36 {
        optional version: VarInt = 1,
        repeated Stats: Proto = 4 {
            optional   idx: VarInt = 1,
            optional delta: VarInt = 2,
        },
        optional user_id: VarInt = 5,
        optional     crc: VarInt = 6,
    },
    Unknown38 = 38 {

    },
    Unknown46 = 46 {
        unknown1: VarInt = 1,
        unknown2: VarInt = 2,
        unknown3: VarInt = 3,
        unknown4: Length = 4,
        unknown5: Length = 5,
        unknown6: Length = 6,
    },
    Unknown47 = 47 {
        unknown1: VarInt = 1,
        unknown2: VarInt = 2,
        unknown3: Length = 3,
        unknown4: Length = 4,
    },
    ServerRankRevealAll = 50 {

    },
    Unknown69 = 69 {
      optional unknown1:  VarInt = 1,
      optional unknown2: Float32 = 2,
      optional unknown3: Float32 = 3,
      optional unknown4: Float32 = 4,
      optional unknown5:  Length = 5,
      optional unknown6: Float32 = 6,
    },
    Unknown75 = 75 {
        optional unknown1: Length = 1,
    },
});