struct v3f64_t {
    double x,y,z;
};

struct header_t {
    u8 cmd;
    u32 tick;
    u8 slot;
};

struct cmd_header_t {
    s64 flags;
    v3f64_t ang[6];
    s32 inseq, outseq;
    u32 data_len;
};

struct varint_t {
    u8 a;
    if (a & 0x80) {
    u8 b;
    if (b & 0x80) {
    u8 c;
    }
    }
};

fn varint_parse(varint_t vi) {
    u32 value = 0;
    value += vi.a & 0x7F;
    if (vi.a & 0x80) {
        value += (vi.b & 0x7f) << 7;
        if (vi.b & 0x80) {
            value += (vi.c & 0x7f) << 14;
        }
    }
    return value;
};

struct protomsg_t {
    u8 msg;
    
    if (msg != 9) {
    //u8 capture[124];
    }
    
    varint_t len;
    u8 data[varint_parse(len)];
};

struct foo_int_t {
    
    
};

struct foo_t {
    header_t header;
    
    if (header.cmd == 1) {
        cmd_header_t cmd_header;
        u8 data[cmd_header.data_len];
    }
    
    if (header.cmd == 6) {
        u32 data_len;
        protomsg_t proto_msgs[490];
        
        u8 dataless[data_len - sizeof(proto_msgs)];
    }
};

foo_t bar[4] @ 0x0;


