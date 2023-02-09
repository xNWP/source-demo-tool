use std::io::Read;

use crate::{
    engine_types::Vector3F64,
    event_data::EventData,
    parse_tools::{ parse_i32, parse_i64, },
    protobuf_message::{ProtobufMessage, ProtobufMessageEnumTraits},
};

use buf_redux::BufReader;

pub mod netmessage;
pub mod usermessage;
pub mod protobuf_value;

#[derive(Debug, Clone)]
pub struct PacketData {
    pub header: Header,
    pub network_messages: Vec<MessageParseReturn<netmessage::NetMessage>>
}

impl PacketData {
    pub fn from_packet_index(packet_index: PacketIndex) -> Self {
        let header = packet_index.header;

        let mut data_reader = BufReader::with_capacity
            (packet_index.data.len(), packet_index.data.as_slice());

        data_reader.read_into_buf().unwrap();

        let mut network_messages = Vec::new();
        while data_reader.buf_len() > 0 {
            let net_result = netmessage::NetMessage::parse_from_bufredux_reader(&mut data_reader);

            let mut message = None;
            let mut warnings = None;
            let mut err = None;
            if let Ok((msg, warn)) = net_result {
                message = Some(msg);
                warnings = Some(warn);
            } else {
                err = Some(net_result.unwrap_err());
            }

            network_messages.push(MessageParseReturn{ message, warnings, err });
        }

        Self { header, network_messages }
    }
}

#[derive(Debug, Clone)]
pub struct Header {
     pub command_info: CommandInfo,
     pub in_seq: i32,
     pub out_seq: i32,
     pub data_length: i32
}

impl Header {
    pub fn from_readable(mut reader: impl Read) -> Result<Self, String> {
        let command_info = match CommandInfo::from_readable(&mut reader) {
            Ok(ci) => ci,
            Err(e) => return Err(format!("error occured parsing command info: {e}"))
        };

        let in_seq = match parse_i32(&mut reader) {
            Ok(n) => n,
            Err(e) => return Err(format!("error occured reading in sequence: {e}"))
        };

        let out_seq = match parse_i32(&mut reader) {
            Ok(n) => n,
            Err(e) => return Err(format!("error occured reading out sequence: {e}"))
        };

        let data_length = match parse_i32(&mut reader) {
            Ok(n) => n,
            Err(e) => return Err(format!("error occured reading data length: {e}"))
        };

        Ok(Header {
            command_info,
            in_seq,
            out_seq,
            data_length
        })
    }
}

pub struct PacketIndex {
    pub header: Header,
    pub data: Vec<u8>
}

impl PacketIndex {
    pub fn from_readable(mut reader: impl Read) -> Result<Self, String> {
        let header = Header::from_readable(&mut reader)?;

        let mut data = Vec::new();
        data.resize(header.data_length.try_into().unwrap(), 0);

        match reader.read_exact(data.as_mut_slice()) {
            Ok(()) => Ok(PacketIndex{header, data}),
            Err(_) => Err(String::from("couldn't read data into buffer"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub flags: i64,
    pub view_origin: Vector3F64,
    pub view_angles: Vector3F64,
    pub local_view_angles: Vector3F64,
    pub inter_view_origin: Vector3F64,
    pub inter_view_angles: Vector3F64,
    pub inter_local_view_angles: Vector3F64
}

impl CommandInfo {
    fn from_readable(mut reader: impl Read) -> Result<CommandInfo, String> {
        let flags = match parse_i64(&mut reader) {
            Ok(n) => n,
            Err(e) => return Err(format!("error parsing flags: {e}"))
        };

        let view_origin = match Vector3F64::from_readable(&mut reader) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing view origin: {e}"))
        };

        let view_angles = match Vector3F64::from_readable(&mut reader) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing view angles: {e}"))
        };

        let local_view_angles = match Vector3F64::from_readable(&mut reader) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing local view angles: {e}"))
        };

        let inter_view_origin = match Vector3F64::from_readable(&mut reader) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing interpolated view origin: {e}"))
        };

        let inter_view_angles = match Vector3F64::from_readable(&mut reader) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing interpolated view angles: {e}"))
        };

        let inter_local_view_angles = match Vector3F64::from_readable(&mut reader) {
            Ok(v) => v,
            Err(e) => return Err(format!("error parsing interpolated local view angles: {e}"))
        };

        Ok(CommandInfo {
            flags,
            view_origin,
            view_angles,
            local_view_angles,
            inter_view_origin,
            inter_view_angles,
            inter_local_view_angles
        })
    }
}

#[derive(Debug, Clone)]
pub enum ParseMessageErr {
    InvalidOrCorrupt(EventData),
    UnknownCommand(u64),
}

#[derive(Debug, Clone)]
pub struct FromProtobufMessagesWarnings {
    pub unknown_fields: Vec<ProtobufMessage>,
    pub missing_fields: Vec<(u8, &'static str)>,
    pub sub_warnings: Vec<(&'static str, FromProtobufMessagesWarnings)>
}

impl FromProtobufMessagesWarnings {
    pub fn has_warnings(&self) -> bool {
        if !self.unknown_fields.is_empty() || !self.missing_fields.is_empty() {
            return true;
        }
        for sub_warn in &self.sub_warnings {
            if sub_warn.1.has_warnings() {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct MessageParseReturn<MessageType: ProtobufMessageEnumTraits> {
    pub message: Option<MessageType>,
    pub warnings: Option<FromProtobufMessagesWarnings>,
    pub err: Option<ParseMessageErr>
}