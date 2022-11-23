#![allow(dead_code)]
//! Access Valve's Server Query using this package.
//!
//! # Game Server Info
//!
//! ```no_run
//! use valve_server_query::Server;
//!
//! let server = Server::new("127.0.0.1:12345").expect("Connect to dedicated server running Valve game");
//!
//! let info = server.info().expect("Get general server information");
//! let players = server.players().expect("Get server player information");
//! let rules = server.rules().expect("Get server rules");
//! ```

pub use models::info::Info;
pub use models::info::Platform;
pub use models::info::ServerType;
pub use models::info::Vac;
pub use models::info::Visibility;
pub use models::Player;
pub use server::Server;
pub use server::Rules;

const ENCODING: &str = "utf-8";
const PACKET_SIZE: usize = 1400;
/// Packet is not split.
const SIMPLE_RESPONSE_HEADER: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
/// Packet is split.
const MULTI_PACKET_RESPONSE_HEADER: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFE];

/// All types are little endian
pub mod types {

    // All types are little endian
    pub type Byte = u8;
    pub type Short = i16;
    pub type Long = i32;
    pub type Float = f32;
    pub type LongLong = u64;
    pub type CString = std::ffi::CString;

    /// All types are little endian,
    pub enum DataType {
        // Name   Description
        //
        // byte   8 bit character or unsigned integer
        // short  16 bit signed integer
        // long   32 bit signed integer
        // float  32 bit floating point
        // long   long 64 bit unsigned integer
        // string variable-length byte field, encoded in UTF-8, terminated by null byte (0x00)
        Byte(Byte),
        Short(Short),
        Long(i32),
        Float(f32),
        LongLong(u64),
        // UTF-8 Encoded
        // Null-Terminated
        String(CString),
    }

    type ByteVec = Vec<u8>;

    pub fn get_byte<'a, I>(bytes: &mut I) -> Byte
    where
        I: Iterator<Item = &'a u8>,
    {
        bytes.next().expect("the next byte exists").to_owned()
    }
    pub fn get_short<'a, I>(bytes: &mut I) -> Short
    where
        I: Iterator<Item = &'a u8>,
    {
        Short::from_le_bytes([*bytes.next().expect("next byte exists"), *bytes.next().expect("next byte exists")])
    }
    pub fn get_long<'a, I>(bytes: &mut I) -> Long
    where
        I: Iterator<Item = &'a u8>,
    {
        Long::from_le_bytes([
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
        ])
    }
    pub fn get_float<'a, I>(bytes: &mut I) -> Float
    where
        I: Iterator<Item = &'a u8>,
    {
        Float::from_le_bytes([
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
        ])
    }
    pub fn get_longlong<'a, I>(bytes: &mut I) -> LongLong
    where
        I: Iterator<Item = &'a u8>,
    {
        LongLong::from_le_bytes([
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
            *bytes.next().expect("next byte exists"),
        ])
    }
    pub fn get_string<'a, I>(bytes: &mut I) -> String
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut string = String::new();
        loop {
            let byte = bytes.next().expect("next byte exists");
            if *byte == 0 {
                break;
            } else {
                string.push(*byte as char);
            }
        }
        string
    }
}

pub mod models {

    use crate::types::{get_byte, get_float, get_long, get_string, Byte, Float, Long};

    #[derive(Debug, PartialEq, Clone)]
    pub struct Player {
        index: Byte,
        name: String,
        score: Long,
        duration: Float,
    }

    impl Default for Player {
        fn default() -> Self {
            Self {
                index: 0,
                name: "".to_string(),
                score: 0,
                duration: 0.0,
            }
        }
    }

    impl Player {
        pub fn get_players(bytes: &[u8]) -> Vec<Self> {
            let mut it = bytes.iter();
            let mut players: Vec<Self> = Vec::new();

            while it.len()
                > (
                    // There's a String too, but that has a varialble size.
                    std::mem::size_of::<Byte>()
                        + std::mem::size_of::<Long>()
                        + std::mem::size_of::<Float>()
                )
            {
                let player = Self::from_iter_bytes(&mut it);

                players.push(player);
            }

            players
        }

        pub fn from_iter_bytes<'a, I>(iter_bytes: &mut I) -> Self
        where
            I: Iterator<Item = &'a u8>,
        {
            let index = get_byte(iter_bytes);
            let name = get_string(iter_bytes);
            let score = get_long(iter_bytes);
            let duration = get_float(iter_bytes);

            Self {
                index,
                name,
                score,
                duration,
            }
        }

        pub fn from_bytes(bytes: &[u8]) -> Self {
            let mut it = bytes.iter();

            let index = get_byte(&mut it);
            let name = get_string(&mut it);
            let score = get_long(&mut it);
            let duration = get_float(&mut it);

            Self {
                index,
                name,
                score,
                duration,
            }
        }
    }

    pub mod info {

        use crate::types::{Byte, LongLong, Short};

        /// Represents a steam game server.
        ///
        /// Ref: <https://developer.valvesoftware.com/wiki/Server_queries#A2S_INFO>
        ///
        /// ```compile_fail
        /// let server_name = info.name();
        /// let loaded_map = info.map();
        /// let max_players = info.player_max();
        /// let players_online = info.player_count();
        /// ```
        #[derive(Debug, PartialEq, Clone)]
        pub struct Info {
            /// Response header. Always equal to 'I' (0x49).
            header: Byte,
            /// Protocol version used by the server.
            protocol: Byte,
            /// Name of the server.
            name: String,
            /// Map the server has currently loaded.
            map: String,
            /// Name of the folder containing the game files.
            folder: String,
            /// Full name of the game.
            game: String,
            /// Steam Application ID of game.
            id: Short,
            /// Number of players on the server.
            players: Byte,
            /// Maximum number of players the server reports it can hold.
            max_players: Byte,
            /// Number of bots on the server.
            bots: Byte,
            /// Indicates the type of server:
            /// 'd' for a dedicated server
            /// 'l' for a non-dedicated server
            /// 'p' for a SourceTV relay (proxy)
            server_type: ServerType,
            /// Indicates the operating system of the server:
            /// 'l' for Linux
            /// 'w' for Windows
            /// 'm' or 'o' for Mac (the code changed after L4D1)
            environment: Platform,
            /// Indicates whether the server requires a password:
            /// 0 for public
            /// 1 for private
            visibility: Visibility,
            /// Specifies whether the server uses VAC:
            /// 0 for unsecured
            /// 1 for secured
            vac: Vac,
            /// Version of the game installed on the server.
            game_version: String,
            /// Flag for Extra Features
            extra_data_flag: Option<Byte>,
            /// The server's game port number.
            port: Option<Short>,
            /// Server's SteamID.
            steam_id: Option<LongLong>,
            /// Spectator port number for SourceTV.
            spectator_port: Option<Short>,
            /// Name of the spectator server for SourceTV.
            spectator_name: Option<String>,
            /// Tags that describe the game according to the server (for future use.)
            keywords: Option<String>,
            /// The server's 64-bit GameID. If this is present, a more accurate AppID is present in the
            /// low 24 bits. The earlier AppID could have been truncated as it was forced into 16-bit
            /// storage.
            game_id: Option<LongLong>,
            /// Trailing bytes for Self::from_bytes
            trailing_bytes: Option<Vec<Byte>>,
        }

        impl Info {
            pub fn from_bytes(bytes: &[u8]) -> Self {
                use crate::types::get_byte;
                use crate::types::get_longlong;
                use crate::types::get_short;
                use crate::types::get_string;
                use crate::utils::compress_trailing_null_bytes;

                let mut it = bytes.iter();

                let header = get_byte(&mut it);
                let protocol = get_byte(&mut it);
                let name = get_string(&mut it);
                let map = get_string(&mut it);
                let folder = get_string(&mut it);
                let game = get_string(&mut it);
                let id = get_short(&mut it);
                let players = get_byte(&mut it);
                let max_players = get_byte(&mut it);
                let bots = get_byte(&mut it);
                let server_type = ServerType::from_byte(&get_byte(&mut it));
                let environment = Platform::from_byte(&get_byte(&mut it));
                let visibility = Visibility::from_byte(&get_byte(&mut it));
                let vac = Vac::from_byte(&get_byte(&mut it));
                let game_version = get_string(&mut it);

                let extra_data_flag: Option<u8>;
                if let Some(u) = it.next() {
                    extra_data_flag = Some(*u);
                } else {
                    extra_data_flag = None;
                }

                let port: Option<Short>;
                if extra_data_flag.is_some() && (extra_data_flag.expect("data exists") & 0x80) != 0 {
                    port = Some(get_short(&mut it));
                } else {
                    port = None;
                }

                let steam_id: Option<LongLong>;
                if extra_data_flag.is_some() && (extra_data_flag.expect("data exists") & 0x10) != 0 {
                    steam_id = Some(get_longlong(&mut it));
                } else {
                    steam_id = None;
                }

                let spectator_port: Option<Short>;
                let spectator_name: Option<String>;
                if extra_data_flag.is_some() && (extra_data_flag.expect("data exists") & 0x40) != 0 {
                    spectator_port = Some(get_short(&mut it));
                    spectator_name = Some(get_string(&mut it));
                } else {
                    spectator_port = None;
                    spectator_name = None;
                }

                let keywords: Option<String>;
                if extra_data_flag.is_some() && (extra_data_flag.expect("data exists") & 0x20) != 0 {
                    keywords = Some(get_string(&mut it));
                } else {
                    keywords = None;
                }

                let game_id: Option<LongLong>;
                if extra_data_flag.is_some() && (extra_data_flag.expect("data exists") & 0x01) != 0 {
                    game_id = Some(get_longlong(&mut it));
                } else {
                    game_id = None;
                }

                // These are hanging bytes that were not parsed
                let trailing_bytes: Option<Vec<u8>> = if it.len() > 0 {
                    // Remove trailing null bytes (and leave one if there are any)
                    let mut min_bytes: Vec<u8> = it.into_iter().map(|x| *x).collect();
                    compress_trailing_null_bytes(&mut min_bytes);

                    // Just a [0]
                    if min_bytes.len() == 1 && *min_bytes.last().expect("last byte exists") == 0 {
                        None
                    } else {
                        Some(min_bytes.into_iter().collect::<Vec<u8>>())
                    }
                } else {
                    None
                };

                Self {
                    header,
                    game_id,
                    trailing_bytes,
                    keywords,
                    spectator_port,
                    spectator_name,
                    extra_data_flag,
                    steam_id,
                    protocol,
                    name,
                    map,
                    folder,
                    game,
                    id,
                    players,
                    max_players,
                    bots,
                    server_type,
                    environment,
                    visibility,
                    vac,
                    game_version,
                    port,
                }
            }
        }

        /// Getters (Immutable)
        impl Info {
            /// Name of the server.
            pub fn name(&self) -> &str {
                &self.name
            }
            /// Map the server has currently loaded.
            pub fn map(&self) -> &str {
                &self.map
            }
            /// Name of the folder containing the game files.
            pub fn folder(&self) -> &str {
                &self.folder
            }
            /// Tags that describe the game according to the server (for future use)
            // TODO: Current server has CSV format. Is this consistent?
            pub fn keywords(&self) -> &Option<String> {
                &self.keywords
            }

            /// Full name of the game.
            pub fn game(&self) -> &str {
                &self.game
            }
            /// The server's 64-bit GameID. If this is present, a more accurate AppID is present in the
            /// low 24 bits. The earlier AppID could have been truncated as it was forced into 16-bit
            /// storage.
            pub fn game_id(&self) -> &Option<LongLong> {
                &self.game_id
            }
            /// Version of the game installed on the server.
            pub fn game_version(&self) -> &str {
                &self.game_version
            }
            /// Steam Application ID of game
            pub fn steam_app_id(&self) -> &Short {
                &self.id
            }
            /// Server's SteamID.
            pub fn steam_id(&self) -> &Option<LongLong> {
                &self.steam_id
            }

            /// Number of players on the server.
            pub fn player_count(&self) -> &Byte {
                &self.players
            }
            /// Maximum number of players the server reports it can hold.
            pub fn player_max(&self) -> &Byte {
                &self.max_players
            }
            /// Number of bots on the server.
            pub fn bot_count(&self) -> &Byte {
                &self.bots
            }

            /// Indicates the type of server
            pub fn server_type(&self) -> &ServerType {
                &self.server_type
            }
            /// Indicates the operating system of the server
            pub fn platform(&self) -> &Platform {
                &self.environment
            }

            /// Indicates whether the server requires a password
            pub fn visibility(&self) -> &Visibility {
                &self.visibility
            }
            /// Specifies whether the server uses VAC
            pub fn vac(&self) -> &Vac {
                &self.vac
            }

            /// The server's game port number
            pub fn port(&self) -> &Option<Short> {
                &self.port
            }
            /// Name of the spectator server for SourceTV.
            pub fn spectator_name(&self) -> &Option<String> {
                &self.spectator_name
            }
            /// Spectator port number for SourceTV.
            pub fn spectator_port(&self) -> &Option<Short> {
                &self.spectator_port
            }
        }

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub enum ServerType {
            Dedicated,
            NonDedicated,
            SourceTvRelay,
        }

        impl ServerType {
            fn from_byte(byte: &u8) -> Self {
                use self::ServerType::{Dedicated, NonDedicated, SourceTvRelay};

                match *byte as char {
                    'd' => Dedicated,
                    'l' => NonDedicated,
                    'p' => SourceTvRelay,
                    _ => panic!("Unrecognized Server Type: <{byte}>."),
                }
            }
        }

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub enum Platform {
            Linux,
            Windows,
            Mac,
        }

        impl Platform {
            fn from_byte(byte: &u8) -> Self {
                use self::Platform::{Linux, Mac, Windows};

                match *byte as char {
                    'l' => Linux,
                    'w' => Windows,
                    'm' => Mac,
                    'o' => Mac,
                    _ => panic!("Unrecognized Environment: <{byte}>."),
                }
            }
        }

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub enum Visibility {
            Public,
            Private,
        }

        impl Visibility {
            fn from_byte(byte: &u8) -> Self {
                use self::Visibility::{Private, Public};

                match *byte {
                    0x00 => Public,
                    0x01 => Private,
                    _ => panic!("Unrecognized Visibility Byte: <{byte}>."),
                }
            }
        }

        #[derive(Debug, Eq, PartialEq, Clone)]
        /// Specifies if a server uses VAC.
        pub enum Vac {
            Unsecured,
            Secured,
        }

        impl Vac {
            fn from_byte(byte: &u8) -> Self {
                use self::Vac::{Secured, Unsecured};

                match *byte {
                    0x00 => Unsecured,
                    0x01 => Secured,
                    _ => panic!("Unrecognized Vac Byte: <{byte}>."),
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_servertype_from_byte() {
                assert_eq!(ServerType::Dedicated, ServerType::from_byte(&('d' as u8)));
            }
            #[test]
            fn test_environment_from_byte() {
                assert_eq!(Platform::Linux, Platform::from_byte(&('l' as u8)));
            }
            #[test]
            fn test_visibility_from_byte() {
                assert_eq!(Visibility::Public, Visibility::from_byte(&(0x00)));
            }
            #[test]
            fn test_vac_from_byte() {
                assert_eq!(Vac::Secured, Vac::from_byte(&(0x01)));
            }
        }
    }
}

pub mod server {

    use crate::{MULTI_PACKET_RESPONSE_HEADER, PACKET_SIZE, SIMPLE_RESPONSE_HEADER};
    use std::collections::HashMap;
    use std::error::Error;
    use std::io;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
    use std::time::Duration;

    use crate::models::info::Info;
    use crate::models::Player;
    use crate::types::Byte;
    use crate::utils::get_multipacket_data;

    pub type Rules = HashMap<String, String>;

    /// Represents a game server running a Steam game.
    ///
    /// ```compile_fail
    /// let server = Server::new("127.0.0.1:12345").expect("Connect to dedicated server running Valve game");
    ///
    /// let info = server.info().expect("Get general server information");
    /// let players = server.players().expect("Get server player information");
    /// let rules = server.rules().expect("Get server rules");
    /// ```
    #[derive(Debug)]
    pub struct Server {
        socket: UdpSocket,
        addr: SocketAddr,
    }

    impl Server {
        pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
            // Init
            let addr: SocketAddr;
            let socket: UdpSocket;

            // Handle Errors
            let result: Result<SocketAddr, _> = url.parse();
            if let Ok(a) = result {
                addr = a;
            } else {
                if let Err(e) = result {
                    return Err(Box::new(e));
                } else {
                    unreachable!();
                }
            }

            let result: Result<UdpSocket, _> =
                UdpSocket::bind((IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));
            if let Ok(s) = result {
                socket = s;
            } else {
                if let Err(e) = result {
                    return Err(Box::new(e));
                } else {
                    unreachable!();
                }
            }

            // Socket Settings
            socket
                .set_read_timeout(Some(Duration::from_secs(1)))?;
            socket
                .set_write_timeout(Some(Duration::from_secs(1)))?;

            // Return Successfully
            Ok(Self { addr, socket })
        }
    }

    /// Socket Settings
    impl Server {
        pub fn set_read_timeout(&mut self, duration: Option<Duration>) -> Result<(), Box<dyn Error>> {
            self.socket.set_read_timeout(duration)?;
            Ok(())
        }
        pub fn set_write_timeout(&mut self, duration: Option<Duration>) -> Result<(), Box<dyn Error>> {
            self.socket.set_write_timeout(duration)?;
            Ok(())
        }
    }

    // A2S_INFO Implementation
    impl Server {
        pub fn info(&self) -> Result<Info, io::Error> {
            let mut request: Vec<u8> = vec![
                255, 255, 255, 255, 84, 83, 111, 117, 114, 99, 101, 32, 69, 110, 103, 105, 110,
                101, 32, 81, 117, 101, 114, 121, 0,
            ];

            self.socket.send_to(&request, &self.addr)?;

            let mut buffer = [0; PACKET_SIZE];
            let mut bytes_returned = self.socket.recv(&mut buffer)?;

            if bytes_returned == 9 {
                // Challenge Received

                // Last 5 bytes of the response
                let challenge = buffer
                    .into_iter()
                    .rev()
                    .skip_while(|&i| i == 0)
                    .collect::<Vec<u8>>()
                    .to_owned()
                    .into_iter()
                    .rev()
                    .collect::<Vec<u8>>()[5..]
                    .to_vec();

                request.extend(challenge);

                self.socket.send_to(&request, &self.addr)?;
                buffer = [0; PACKET_SIZE];
                bytes_returned = self.socket.recv(&mut buffer)?;
            }

            let packet_header = &buffer[..4];
            let payload: Vec<u8>;

            if packet_header == SIMPLE_RESPONSE_HEADER {
                payload = buffer[4..bytes_returned + 1].to_vec();
            } else if packet_header == MULTI_PACKET_RESPONSE_HEADER {
                // id starts at 0
                // tcp means they don't have to be in order
                let (_answer_id, total, packet_id) = get_multipacket_data(&buffer);
                let mut packet_map: HashMap<Byte, Vec<u8>> = HashMap::with_capacity(total as usize);

                let current_payload = buffer[(4 + 4 + 1 + 1)..bytes_returned + 1].to_vec();
                packet_map.insert(packet_id, current_payload);

                // Get the remaining packet data.
                while total > packet_map.len() as u8 {
                    buffer = [0; PACKET_SIZE]; // Clear buffer
                    bytes_returned = self.socket.recv(&mut buffer)?;

                    let (_answer_id, _total, packet_id) = get_multipacket_data(&buffer);
                    let current_payload = buffer[(4 + 4 + 1 + 1)..bytes_returned + 1].to_vec();
                    packet_map.insert(packet_id, current_payload);
                }

                // Sort and Collect all packet data
                let mut v: Vec<(u8, Vec<u8>)> = packet_map.into_iter().collect();
                v.sort_by_key(|i| i.0);
                payload = v
                    .into_iter()
                    .map(|(_, bytes)| bytes)
                    .flatten()
                    .collect::<Vec<u8>>();
            } else {
                panic!("An unknown packet header was received.");
            }

            let info = Info::from_bytes(&payload);
            Ok(info)
        }
    }

    // A2S_PLAYER Implementation
    impl Server {
        pub fn players(&self) -> Result<Vec<Player>, io::Error> {
            let request = [
                0xFF, 0xFF, 0xFF, 0xFF, // Simple Header
                0x55, // Header
                0xFF, 0xFF, 0xFF, 0xFF, // Request Challenge
            ];

            self.socket.send_to(&request, &self.addr)?;

            let mut buffer = [0; PACKET_SIZE];
            let _bytes_returned = self.socket.recv(&mut buffer)?;

            //  Get Challenge
            let challenge = buffer
                .into_iter()
                .rev()
                .skip_while(|&i| i == 0)
                .collect::<Vec<u8>>()
                .to_owned()
                .into_iter()
                .rev()
                .collect::<Vec<u8>>()[5..]
                .to_vec();

            // Resend Request
            let mut request = vec![
                0xFF, 0xFF, 0xFF, 0xFF, // Simple Header
                0x55, // Header
            ];
            request.extend(challenge);

            // Get Data
            self.socket.send_to(&request, &self.addr)?;
            buffer = [0; PACKET_SIZE];
            let mut bytes_returned = self.socket.recv(&mut buffer)?;

            // Parse Data
            let packet_header = &buffer[..=3];

            let payload: Vec<u8>;
            if packet_header == SIMPLE_RESPONSE_HEADER {
                payload = buffer[4..].to_vec();
            } else if packet_header == MULTI_PACKET_RESPONSE_HEADER {
                // id starts at 0
                // tcp means they don't have to be in order
                let (_answer_id, total, packet_id) = get_multipacket_data(&buffer);
                let mut packet_map: HashMap<Byte, Vec<u8>> = HashMap::with_capacity(total as usize);

                let current_payload = buffer[(4 + 4 + 1 + 1)..bytes_returned + 1].to_vec();
                packet_map.insert(packet_id, current_payload);

                // Get the remaining packet data.
                while total > packet_map.len() as u8 {
                    buffer = [0; PACKET_SIZE]; // Clear buffer
                    bytes_returned = self.socket.recv(&mut buffer)?;

                    let (_answer_id, _total, packet_id) = get_multipacket_data(&buffer);
                    let current_payload = buffer[(4 + 4 + 1 + 1)..bytes_returned + 1].to_vec();
                    packet_map.insert(packet_id, current_payload);
                }

                // Sort and Collect all packet data
                let mut v: Vec<(u8, Vec<u8>)> = packet_map.into_iter().collect();
                v.sort_by_key(|i| i.0);
                payload = v
                    .into_iter()
                    .map(|(_, bytes)| bytes)
                    .flatten()
                    .collect::<Vec<u8>>();
            } else {
                panic!("An unknown packet header was received.");
            }

            let _header: &Byte = &payload[0];
            let _player_count: Byte = buffer[1].clone();

            let players: Vec<Player> = Player::get_players(&buffer[2..bytes_returned + 1]);

            Ok(players)
        }
    }

    /// A2S_RULES Implementation
    impl Server {
        pub fn rules(&self) -> Result<Rules, io::Error> {
            use crate::utils::compress_trailing_null_bytes;

            let request = [
                0xFF, 0xFF, 0xFF, 0xFF, // Simple Header
                0x56, // Header
                0xFF, 0xFF, 0xFF, 0xFF, // Request Challenge
            ];

            self.socket.send_to(&request, &self.addr)?;

            let mut buffer = [0; PACKET_SIZE];
            let _bytes_returned = self.socket.recv(&mut buffer)?;

            //  Get Challenge
            let challenge = buffer
                .into_iter()
                .rev()
                .skip_while(|&i| i == 0)
                .collect::<Vec<u8>>()
                .to_owned()
                .into_iter()
                .rev()
                .collect::<Vec<u8>>()[5..]
                .to_vec();

            // Resend Request
            let mut request = vec![
                0xFF, 0xFF, 0xFF, 0xFF, // Simple Header
                0x56, // Header
            ];
            request.extend(challenge);

            // Get Data
            self.socket.send_to(&request, &self.addr)?;
            buffer = [0; PACKET_SIZE];
            let mut bytes_returned = self.socket.recv(&mut buffer)?;

            // Parse Data
            let packet_header = &buffer[..=3];
            let _header: &Byte = &buffer[4];

            let mut payload: Vec<u8>;
            if packet_header == SIMPLE_RESPONSE_HEADER {
                let _rule_count: Byte = buffer[5].clone();
                let _ = buffer[6]; // Null Byte
                payload = buffer[7..].to_vec();
                compress_trailing_null_bytes(&mut payload);
            } else if packet_header == MULTI_PACKET_RESPONSE_HEADER {
                // id starts at 0
                // tcp means they don't have to be in order
                let (_answer_id, total, packet_id) = get_multipacket_data(&buffer);
                let mut packet_map: HashMap<Byte, Vec<u8>> = HashMap::with_capacity(total as usize);

                let current_payload = buffer[(4 + 4 + 1 + 1)..bytes_returned + 1].to_vec();
                packet_map.insert(packet_id, current_payload);

                // Get the remaining packet data.
                while total > packet_map.len() as u8 {
                    buffer = [0; PACKET_SIZE]; // Clear buffer
                    bytes_returned = self.socket.recv(&mut buffer)?;

                    let (_answer_id, _total, packet_id) = get_multipacket_data(&buffer);
                    let current_payload = buffer[(4 + 4 + 1 + 1)..bytes_returned + 1].to_vec();
                    packet_map.insert(packet_id, current_payload);
                }

                // Sort and Collect all packet data
                let mut v: Vec<(u8, Vec<u8>)> = packet_map.into_iter().collect();
                v.sort_by_key(|i| i.0);
                payload = v
                    .into_iter()
                    .map(|(_, bytes)| bytes)
                    .flatten()
                    .collect::<Vec<u8>>();
            } else {
                panic!("An unknown packet header was received.");
            }

            let rules: Rules = Self::get_rules(&payload);

            Ok(rules)
        }

        pub fn get_rules(bytes: &[u8]) -> Rules {
            use crate::types::get_string;

            let mut it = bytes.iter();
            let mut rules = HashMap::new();

            while it.len() > 0 {
                let name = get_string(&mut it);
                let value = get_string(&mut it);

                rules.insert(name, value);
            }

            rules
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_client_init() {
            let server: Result<_, _> = Server::new("");
            if let Err(_) = server {
            } else {
                assert!(false, "Server was successfully contructed when it should have failed when parsing URL.")
            }
        }

        #[test]
        #[ignore]
        fn test_client_init_live() {
            // Live server I own
            let server: Result<_, _> = Server::new("54.186.150.6:9879");
            if let Ok(_) = server {
            } else {
                assert!(
                    false,
                    "Server failed to be contructed when it should have succeeded (LIVE TEST)."
                )
            }
        }

        #[test]
        fn test_client_info() {
            // Dummy
            let server = Server::new("127.0.0.1:12345").unwrap();
            let info: Result<Info, _> = server.info();
            if let Err(_) = info {
            } else {
                assert!(
                    false,
                    "Target URL is not real, but we got back an Ok response for A2S_INFO."
                )
            }
        }

        #[test]
        #[ignore]
        fn test_client_info_live() {
            // Live server I own
            let server = Server::new("54.186.150.6:9879").unwrap();
            let info: Result<Info, _> = server.info();
            if let Ok(_) = info {
            } else {
                assert!(
                    false,
                    "Target URL is real and live, but we got back an Err response for A2S_INFO."
                )
            }
        }
        #[test]
        #[ignore]
        fn test_client_players_live() {
            // Live server I own
            let server = Server::new("54.186.150.6:9879").unwrap();
            let players: Result<Vec<Player>, _> = server.players();
            if let Ok(_) = players {
            } else {
                assert!(
                    false,
                    "Target URL is real and live, but we got back an Err response for A2S_PLAYER."
                )
            }
        }
        #[test]
        #[ignore]
        fn test_client_rules_live() {
            // Live server I own
            let server = Server::new("54.186.150.6:9879").unwrap();
            let rules: Result<Rules, _> = server.rules();
            if let Ok(_) = rules {
            } else {
                assert!(
                    false,
                    "Target URL is real and live, but we got back an Err response for A2S_RULES."
                )
            }
        }
    }
}

pub mod utils {
    use crate::types::{get_byte, get_long, Byte, Long};

    pub fn get_multipacket_data(buffer: &[u8]) -> (Long, Byte, Byte) {
        let v = buffer.to_vec();
        let mut buffer_mut = v.iter();

        let _header = get_long(&mut buffer_mut);
        let answer_id = get_long(&mut buffer_mut);
        let total = get_byte(&mut buffer_mut);
        let packet_id = get_byte(&mut buffer_mut);

        (answer_id, total, packet_id)
    }

    pub fn compress_trailing_null_bytes(bytes: &mut Vec<u8>) {
        // No Size
        if bytes.len() == 0 || bytes.len() == 1 {
            return;
        }
        // No trailing null bytes
        if bytes.last().expect("a last byte exists") != &0 {
            return;
        }

        // Remove trailing null bytes, then add one null byte
        let mut last = bytes.pop().expect("the next byte exists");
        while last == 0 && bytes.len() > 0 {
            last = bytes.pop().expect("the next byte exists");
        }
        bytes.push(last);
        bytes.push(0x00);
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_compress_null_bytes_basic() {
            let mut bytes: Vec<u8> = vec![1, 2, 3, 0, 0, 0, 0];
            compress_trailing_null_bytes(&mut bytes);

            let result = bytes;
            let expected: Vec<u8> = vec![1, 2, 3, 0];

            assert_eq!(result, expected);
        }
        #[test]
        fn test_compress_null_bytes_with_no_trailing_zeroes() {
            let mut bytes: Vec<u8> = vec![1, 2, 3];
            compress_trailing_null_bytes(&mut bytes);

            let result = bytes;
            let expected: Vec<u8> = vec![1, 2, 3];

            assert_eq!(result, expected);
        }
        #[test]
        fn test_compress_null_bytes_with_one_trailing_zeroes() {
            let mut bytes: Vec<u8> = vec![1, 2, 3, 0];
            compress_trailing_null_bytes(&mut bytes);

            let result = bytes;
            let expected: Vec<u8> = vec![1, 2, 3, 0];

            assert_eq!(result, expected);
        }
        #[test]
        fn test_compress_null_bytes_with_empty_vector() {
            let mut bytes: Vec<u8> = vec![];
            compress_trailing_null_bytes(&mut bytes);

            let result = bytes;
            let expected: Vec<u8> = vec![];

            assert_eq!(result, expected);
        }
        #[test]
        fn test_compress_null_bytes_with_one_zero_as_vector() {
            let mut bytes: Vec<u8> = vec![0];
            compress_trailing_null_bytes(&mut bytes);

            let result = bytes;
            let expected: Vec<u8> = vec![0];

            assert_eq!(result, expected);
        }
    }
}
