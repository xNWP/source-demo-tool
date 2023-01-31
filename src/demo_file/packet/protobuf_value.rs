pub type ProtobufField = (&'static str, ProtobufValue);

pub enum ProtobufValue {
    None,
    VarInt(u64),
    Length(Vec<u8>),
    String(String),
    Fixed32(u32),
    Float32(f32),
    Proto(Vec<ProtobufField>),
    Repeated(Vec<ProtobufValue>),
}