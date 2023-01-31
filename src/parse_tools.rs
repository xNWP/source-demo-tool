use std::io::Read;

use crate::event_data::EventData;

use source_demo_tool_impl_proc_macros::event;

pub fn parse_i32(mut reader: impl Read) -> Result<i32, &'static str> {
    let mut buffer: [u8; 4] = [0; 4];
    match reader.read_exact(&mut buffer) {
        Ok(()) => Ok(i32::from_le_bytes(buffer)),
        Err(_) => Err("couldn't read requested bytes from buffer")
    }
}

pub fn parse_i64(mut reader: impl Read) -> Result<i64, &'static str> {
    let mut buffer: [u8; 8] = [0; 8];
    match reader.read_exact(&mut buffer) {
        Ok(()) => Ok(i64::from_le_bytes(buffer)),
        Err(_) => Err("couldn't read requested bytes from buffer")
    }
}

pub fn parse_f32(mut reader: impl Read) -> Result<f32, &'static str> {
    let mut buffer: [u8; 4] = [0; 4];
    match reader.read_exact(&mut buffer) {
        Ok(()) => Ok(f32::from_le_bytes(buffer)),
        Err(_) => Err("couldn't read requested bytes from buffer")
    }
}

pub fn parse_f64(mut reader: impl Read) -> Result<f64, &'static str> {
    let mut buffer: [u8; 8] = [0; 8];
    match reader.read_exact(&mut buffer) {
        Ok(()) => Ok(f64::from_le_bytes(buffer)),
        Err(_) => Err("couldn't read requested bytes from buffer")
    }
}

pub fn parse_u8(mut reader: impl Read) -> Result<u8, &'static str> {
    let mut buffer: [u8; 1] = [0; 1];
    match reader.read_exact(&mut buffer) {
        Ok(()) => Ok(u8::from_le_bytes(buffer)),
        Err(_) => Err("couldn't read requested byte from buffer")
    }
}

pub fn parse_u16(mut reader: impl Read) -> Result<u16, &'static str> {
    let mut buffer: [u8; 2] = [0; 2];
    match reader.read_exact(&mut buffer) {
        Ok(()) => Ok(u16::from_le_bytes(buffer)),
        Err(_) => Err("couldn't read requested bytes from buffer")
    }
}

pub fn parse_u32(mut reader: impl Read) -> Result<u32, &'static str> {
    let mut buffer: [u8; 4] = [0; 4];
    match reader.read_exact(&mut buffer) {
        Ok(()) => Ok(u32::from_le_bytes(buffer)),
        Err(_) => Err("couldn't read requested bytes from buffer")
    }
}

pub fn parse_fixed_width_string(mut reader: impl Read, length: usize) -> Result<String, &'static str> {
    let mut vec_buffer: Vec<u8> = Vec::new();
    vec_buffer.resize(length, 0);
    let temp_string = match reader.read_exact(vec_buffer.as_mut_slice()) {
        Ok(()) => match vec_buffer.iter().position(|&c| c == 0) {
            Some(n) => String::from_utf8(vec_buffer[..n].to_vec()),
            None => String::from_utf8(vec_buffer)
        },
        Err(_) => return Err("couldn't read requested bytes from buffer")
    };

    match temp_string {
        Ok(s) => Ok(s),
        Err(_e) => Err("couldn't parse bytes as valid utf-8 string")
    }
}

pub fn parse_cstr(mut reader: impl Read) -> Result<String, &'static str> {
    let mut vec_buffer = Vec::new();
    loop {
        let mut c = [0; 1];
        match reader.read_exact(&mut c) {
            Ok(()) => vec_buffer.push(c[0]),
            Err(_e) => return Err("couldn't read from buffer")
        }

        if c[0] == 0 {
            break;
        }
    }

    match String::from_utf8(vec_buffer[..vec_buffer.len()-1].to_vec()) {
        Ok(s) => Ok(s),
        Err(_e) => Err("couldn't parse bytes as valid utf-8 string")
    }
}

pub enum ParseVarIntExit {
    Ok(u64),
    TooLong(EventData, [u8; 10]),
    BufferErr(EventData)
}

pub fn parse_varint(mut reader: impl Read) -> ParseVarIntExit {
    let mut value: u64 = 0;
    let mut bytes = [0; 10];
    for i in 0..10 {
        let current_byte = match parse_u8(&mut reader) {
            Ok(n) => n,
            Err(_e) => return ParseVarIntExit::BufferErr(event!{"bad u8 read"})
        };
        bytes[i] = current_byte;
        let byte_value = current_byte & 0b0111_1111;
        let byte_value: u64 = byte_value.into();
        value += byte_value << (7 * i);

        if (current_byte & 0b1000_0000) == 0 {
            return ParseVarIntExit::Ok(value)
        }
    }

    ParseVarIntExit::TooLong(event!{"too long"}, bytes)
}