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

struct foo_t {
    header_t header;
    
    if (header.cmd == 1) {
        cmd_header_t cmd_header;
        u8 data[cmd_header.data_len];
    }
    
    if (header.cmd == 6) {
        u32 data_len;
        u8 data[data_len];
    }
};

foo_t bar[4] @ 0x0;

