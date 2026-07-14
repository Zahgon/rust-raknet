pub const RAKNET_PROTOCOL_VERSION: u8 = 10;
pub const RAKNET_PROTOCOL_VERSION_LIST: [u8; 2] = [10, 11];

pub const RAKNET_CLIENT_MTU: u16 = 1400;

pub const RECEIVE_TIMEOUT: i64 = 60000;

pub enum Endian {
    Big,
    Little,
}

pub fn cur_timestamp_millis() -> i64 { panic!("STUB: not implemented") }

pub fn _is_timeout(time: i64, timeout: u64) -> bool { panic!("STUB: not implemented") }
