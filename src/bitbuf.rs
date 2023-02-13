pub struct BitBuf {
    data: Vec<u8>,
    act_dword: u32,
    next_index: usize,
    act_bits_left: u8,
    data_bits: usize,
    bits_read: usize,
}

impl BitBuf {
    pub fn new(data: Vec<u8>) -> Self {
        assert!(data.len() >= 4, "Must be at least 1 dword long.");

        let data_bits = data.len() << 3;
        Self {
            data,
            data_bits,
            act_dword: 0,
            next_index: 0,
            act_bits_left: 0,
            bits_read: 0,
        }
    }

    fn get_next_dword(&mut self) -> bool {
        if self.next_index >= self.data.len() || self.act_bits_left > 0 {
            false
        } else {
            let mut buf: [u8; 4] = [0; 4];
            let bytes_left = self.data.len() - self.next_index;

            if bytes_left >= 4 {
                for i in 0..4 {
                    buf[i] = self.data[i + self.next_index];
                }
                self.next_index += 4;
                self.act_bits_left = 32;
            } else {
                for i in 0..bytes_left {
                    buf[i] = self.data[i + self.next_index];
                }
                self.next_index += bytes_left;
                self.act_bits_left = (bytes_left << 3).try_into().unwrap();
            }
            self.act_dword = u32::from_le_bytes(buf);
            true
        }
    }

    pub fn read_ubit_var(&mut self) -> u32 {
        let mut rval = self.read_ubit_long(6);
        match rval & (16 | 32) {
            16 => rval = (rval & 15) | (self.read_ubit_long(4) << 4),
            32 => rval = (rval & 15) | (self.read_ubit_long(8) << 4),
            48 => rval = (rval & 15) | (self.read_ubit_long(28) << 4),
            _ => {}
        }
        rval
    }

    pub fn read_ubit_long(&mut self, mut numbits: u8) -> u32 {
        assert!(numbits <= 32);
        self.bits_read += numbits as usize;

        if self.act_bits_left >= numbits {
            let rval = self.act_dword & BIT_MASK_TABLE[numbits as usize];

            self.act_bits_left -= numbits;
            if self.act_bits_left > 0 {
                self.act_dword >>= numbits;
            } else {
                self.get_next_dword();
            }

            rval
        } else {
            let mut rval = self.act_dword;
            let bits_read = self.act_bits_left;
            self.act_bits_left = 0;
            numbits -= bits_read;

            assert!(self.get_next_dword());
            rval |= (self.act_dword & BIT_MASK_TABLE[numbits as usize]) << bits_read;

            self.act_dword >>= numbits;
            self.act_bits_left -= numbits;

            rval
        }
    }

    pub fn read_one_bit(&mut self) -> bool {
        self.bits_read += 1;
        if self.read_ubit_long(1) != 0 {
            true
        } else {
            false
        }
    }

    pub fn get_bits_read(&self) -> usize {
        self.bits_read
    }
}

const BIT_MASK_TABLE: [u32; 33] = [
    0,
    (1 << 1) - 1,
    (1 << 2) - 1,
    (1 << 3) - 1,
    (1 << 4) - 1,
    (1 << 5) - 1,
    (1 << 6) - 1,
    (1 << 7) - 1,
    (1 << 8) - 1,
    (1 << 9) - 1,
    (1 << 10) - 1,
    (1 << 11) - 1,
    (1 << 12) - 1,
    (1 << 13) - 1,
    (1 << 14) - 1,
    (1 << 15) - 1,
    (1 << 16) - 1,
    (1 << 17) - 1,
    (1 << 18) - 1,
    (1 << 19) - 1,
    (1 << 20) - 1,
    (1 << 21) - 1,
    (1 << 22) - 1,
    (1 << 23) - 1,
    (1 << 24) - 1,
    (1 << 25) - 1,
    (1 << 26) - 1,
    (1 << 27) - 1,
    (1 << 28) - 1,
    (1 << 29) - 1,
    (1 << 30) - 1,
    (1 << 31) - 1,
    ((1u64 << 32) - 1) as u32,
];