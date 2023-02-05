use std::io::Read;
use buf_redux::BufReader;
use source_demo_tool_impl_proc_macros::event;

use crate::{parse_tools::{parse_varint, parse_u32, ParseVarIntExit}, event_data::EventData, demo_file::packet::protobuf_value::ProtobufValue};

pub trait ProtobufMessageEnumTraits {
    fn to_vec(&self) -> Vec<(&'static str, ProtobufValue)>;
    fn type_count(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ProtobufMessage {
    pub message: WireMessage,
    pub field_number: u8,
}

#[derive(Debug)]
pub enum FromReadableErr {
    BufferErr(EventData),
    InvalidOrCorrupt(EventData),
    UnsupportedWireType(EventData, u8)
}

impl ProtobufMessage {
    pub fn many_from_vec(raw_data: &Vec<u8>) -> Result<Vec<Self>, FromReadableErr> {
        let mut reader = BufReader::with_capacity
            (raw_data.len(), raw_data.as_slice());
        reader.read_into_buf().unwrap();

        let mut msgs = Vec::new();
        while reader.buf_len() > 0 {
            msgs.push(
                match Self::from_readable(&mut reader) {
                    Ok(val) => val,
                    Err(e) => return Err(e)
                }
            );
        }
        Ok(msgs)
    }

    pub fn from_readable(mut reader: impl Read) -> Result<ProtobufMessage, FromReadableErr> {
        let tag = match parse_varint(&mut reader) {
            ParseVarIntExit::Ok(n) => n,
            _ => return Err(FromReadableErr::InvalidOrCorrupt(event!{"bad tag varint read"}))
        };

        let message = match (tag & 0b111) as u8 {
            wire_type::VARINT => WireMessage::VarInt(match parse_varint(&mut reader) {
                ParseVarIntExit::Ok(n) => n,
                _ => return Err(FromReadableErr::InvalidOrCorrupt(event!{"bad varint read, message"}))
            }),
            wire_type::LENGTH => WireMessage::Length({
                let len = match parse_varint(&mut reader) {
                    ParseVarIntExit::Ok(n) => n,
                _ => return Err(FromReadableErr::InvalidOrCorrupt(event!{"bad varint read, len"}))
                };

                let mut v = Vec::new();
                v.resize(len as usize, 0);
                if let Err(_) = reader.read_exact(v.as_mut_slice()) {
                    return Err(FromReadableErr::BufferErr(event!{"length data read"}))
                }
                v
            }),
            wire_type::FIXED32 => WireMessage::Fixed32(match parse_u32(&mut reader) {
                Ok(n) => n,
                Err(e) => {
                    let mut ev = event!{""};
                    ev.details = e.into();
                    return Err(FromReadableErr::InvalidOrCorrupt(ev))
                }
            }),
            n => return Err(FromReadableErr::UnsupportedWireType(event!{"bad wire type"}, n))
        };

        let field_number = ((tag & !0b111) >> 3) as u8;

        Ok(ProtobufMessage { message, field_number })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum WireMessage {
    VarInt(u64),
    Length(Vec<u8>),
    Fixed32(u32)
}
pub mod wire_type {
    pub const VARINT: u8 = 0;
    pub const LENGTH: u8 = 2;
    pub const FIXED32: u8 = 5;
}
