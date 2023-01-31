use std::{io::Read, fmt::Display};
use crate::parse_tools::parse_f64;

#[derive(Debug, Clone)]
pub struct Vector3F64 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3F64 {
    pub fn from_readable(mut reader: impl Read) -> Result<Self, &'static str> {
        let x = parse_f64(&mut reader);
        let y = parse_f64(&mut reader);
        let z = parse_f64(&mut reader);

        if x.or(y.or(z)).is_err() {
            return Err("couldn't parse vector")
        }

        let x = x.unwrap();
        let y = y.unwrap();
        let z = z.unwrap();

        Ok(Self { x, y, z })
    }
}

impl Display for Vector3F64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("< {:.3}, {:.3}, {:.3} >", self.x, self.y, self.z))
    }
}