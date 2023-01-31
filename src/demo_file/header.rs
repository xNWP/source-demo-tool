use std::io::Read;
use crate::parse_tools:: {
    parse_f32,
    parse_i32,
    parse_fixed_width_string
};

#[derive(Debug, Clone)]
pub struct DemoHeader {
	pub demo_protocol: i32,
	pub network_protocol: i32,
	pub server_name: String,
	pub client_name: String,
	pub map_name: String,
	pub game_directory: String,
	pub playback_time: f32,
	pub ticks: i32,
	pub frames: i32,
	pub sign_on_length: i32,
}

impl DemoHeader {
    pub fn from_readable(mut reader: impl Read) -> Result<DemoHeader, String> {
        // parse and check header string
        let mut buffer: [u8; 8] = [0; 8];
        match reader.read_exact(&mut buffer) {
            Ok(()) => match String::from_utf8(buffer.to_vec()) {
                Ok(str) => {
                    if str != "HL2DEMO\0" {
                        return Err(String::from("file magic header != HL2DEMO"))
                    }
                },
                Err(_) => return Err(String::from("couldn't parse header string"))
            },
            Err(_) => return Err(String::from("couldn't read header string"))
        }        
        
        // parse demo protocol
        let demo_protocol = match parse_i32(&mut reader) {
            Ok(n) => {
                if n < 0 {
                    return Err(String::from("unexpected negative demo protocol"))
                }
                n
            }
            Err(e) => return Err(format!("error parsing demo protocol: {e}"))
        };

        // parse network protocol
        let network_protocol = match parse_i32(&mut reader) {
            Ok(n) => {
                if n < 0 {
                    return Err(String::from("unexpected negative network protocol"))
                }
                n
            }
            Err(e) => return Err(format!("error parsing network protocol: {e}"))
        };

        // parse server name
        let server_name = match parse_fixed_width_string(&mut reader, 260) {
            Ok(s) => s,
            Err(e) => return Err(format!("error parsing server name: {e}"))
        };

        // parse client name
        let client_name = match parse_fixed_width_string(&mut reader, 260) {
            Ok(s) => s,
            Err(e) => return Err(format!("error parsing client name: {e}"))
        };

        // parse map name
        let map_name = match parse_fixed_width_string(&mut reader, 260) {
            Ok(s) => s,
            Err(e) => return Err(format!("error parsing map name: {e}"))
        };

        // parse game directory
        let game_directory = match parse_fixed_width_string(&mut reader, 260) {
            Ok(s) => s,
            Err(e) => return Err(format!("error parsing game directory: {e}"))
        };

        // parse playback time
        let playback_time = match parse_f32(&mut reader) {
            Ok(n) => {
                if n < 0.0 {
                    return Err(String::from("unexpected negative playback time"))
                }
                n
            },
            Err(e) => return Err(format!("error parsing playback time: {e}"))
        };

        // parse ticks
        let ticks = match parse_i32(&mut reader) {
            Ok(n) => {
                if n < 0 {
                    return Err(String::from("unexpected negative ticks"))
                }
                n
            },
            Err(e) => return Err(format!("error parsing ticks: {e}"))
        };

        // parse frames
        let frames = match parse_i32(&mut reader) {
            Ok(n) => {
                if n < 0 {
                    return Err(String::from("unexpected negative frames"))
                }
                n
            },
            Err(e) => return Err(format!("error parsing frames: {e}"))
        };

        // parse sign on length
        let sign_on_length = match parse_i32(&mut reader) {
            Ok(n) => {
                if n < 0 {
                    return Err(String::from("unexpected negative sign on length"))
                }
                n
            }
            Err(e) => return Err(format!("error parsing sign on length: {e}"))
        };

        Ok(DemoHeader {
            demo_protocol,
            network_protocol,
            server_name,
            client_name,
            map_name,
            game_directory,
            playback_time,
            ticks,
            frames,
            sign_on_length
        })
    } 
}