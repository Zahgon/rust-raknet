use crate::datatype::{RaknetReader, RaknetWriter};
use crate::error::RaknetError;
use crate::error::*;
use crate::utils::Endian;
use std::net::SocketAddr;

#[warn(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PacketID {
    ConnectedPing = 0x00,
    UnconnectedPing1 = 0x01,
    UnconnectedPing2 = 0x02,
    ConnectedPong = 0x03,
    UnconnectedPong = 0x1c,
    OpenConnectionRequest1 = 0x05,
    OpenConnectionReply1 = 0x06,
    OpenConnectionRequest2 = 0x07,
    OpenConnectionReply2 = 0x08,
    ConnectionRequest = 0x09,
    ConnectionRequestAccepted = 0x10,
    AlreadyConnected = 0x12,
    NewIncomingConnection = 0x13,
    Disconnect = 0x15,
    IncompatibleProtocolVersion = 0x19,
    FrameSetPacketBegin = 0x80,
    FrameSetPacketEnd = 0x8d,
    Nack = 0xa0,
    Ack = 0xc0,
    Game = 0xfe,
}

impl PacketID {
    pub fn to_u8(self) -> u8 { panic!("STUB: not implemented") }

    pub fn from(id: u8) -> Result<Self> { panic!("STUB: not implemented") }
}

macro_rules! unwrap_or_return {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        }
    };
}

#[derive(Clone)]
pub struct ConnectedPing {
    pub client_timestamp: i64,
}

#[derive(Clone)]
pub struct PacketUnconnectedPing {
    pub time: i64,
    pub magic: bool,
    pub guid: u64,
}

#[derive(Clone)]
pub struct PacketUnconnectedPong {
    pub time: i64,
    pub guid: u64,
    pub magic: bool,
    pub motd: String,
}

#[derive(Clone)]
pub struct ConnectedPong {
    pub client_timestamp: i64,
    pub server_timestamp: i64,
}

#[derive(Clone)]
pub struct OpenConnectionRequest1 {
    pub magic: bool,
    pub protocol_version: u8,
    pub mtu_size: u16,
}

#[derive(Clone)]
pub struct OpenConnectionRequest2 {
    pub magic: bool,
    pub address: std::net::SocketAddr,
    pub mtu: u16,
    pub guid: u64,
}

#[derive(Clone)]
pub struct OpenConnectionReply1 {
    pub magic: bool,
    pub guid: u64,
    pub use_encryption: u8,
    pub mtu_size: u16,
}

#[derive(Clone)]
pub struct OpenConnectionReply2 {
    pub magic: bool,
    pub guid: u64,
    pub address: std::net::SocketAddr,
    pub mtu: u16,
    pub encryption_enabled: u8,
}

#[derive(Clone)]
pub struct ConnectionRequest {
    pub guid: u64,
    pub time: i64,
    pub use_encryption: u8,
}

#[derive(Clone)]
pub struct ConnectionRequestAccepted {
    pub client_address: std::net::SocketAddr,
    pub system_index: u16,
    pub request_timestamp: i64,
    pub accepted_timestamp: i64,
}

#[derive(Clone)]
pub struct NewIncomingConnection {
    pub server_address: std::net::SocketAddr,
    pub request_timestamp: i64,
    pub accepted_timestamp: i64,
}

#[derive(Clone)]
pub struct IncompatibleProtocolVersion {
    pub server_protocol: u8,
    pub magic: bool,
    pub server_guid: u64,
}

#[derive(Clone)]
pub struct AlreadyConnected {
    pub magic: bool,
    pub guid: u64,
}

#[derive(Clone)]
pub struct Nack {
    pub record_count: u16,
    pub sequences: Vec<(u32, u32)>,
}

#[derive(Clone)]
pub struct Ack {
    pub record_count: u16,
    pub sequences: Vec<(u32, u32)>,
}

pub fn read_packet_ping(buf: &[u8]) -> Result<PacketUnconnectedPing> { panic!("STUB: not implemented") }

pub fn write_packet_ping(packet: &PacketUnconnectedPing) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_pong(buf: &[u8]) -> Result<PacketUnconnectedPong> { panic!("STUB: not implemented") }

pub fn write_packet_pong(packet: &PacketUnconnectedPong) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_connection_open_request_1(buf: &[u8]) -> Result<OpenConnectionRequest1> { panic!("STUB: not implemented") }

pub fn write_packet_connection_open_request_1(packet: &OpenConnectionRequest1) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_connection_open_request_2(buf: &[u8]) -> Result<OpenConnectionRequest2> { panic!("STUB: not implemented") }

pub fn write_packet_connection_open_request_2(packet: &OpenConnectionRequest2) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_connection_open_reply_1(buf: &[u8]) -> Result<OpenConnectionReply1> { panic!("STUB: not implemented") }

pub fn write_packet_connection_open_reply_1(packet: &OpenConnectionReply1) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_connection_open_reply_2(buf: &[u8]) -> Result<OpenConnectionReply2> { panic!("STUB: not implemented") }

pub fn write_packet_connection_open_reply_2(packet: &OpenConnectionReply2) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn _read_packet_already_connected(buf: &[u8]) -> Result<AlreadyConnected> { panic!("STUB: not implemented") }

pub fn write_packet_already_connected(packet: &AlreadyConnected) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_incompatible_protocol_version(
    buf: &[u8],
) -> Result<IncompatibleProtocolVersion> { panic!("STUB: not implemented") }

pub fn write_packet_incompatible_protocol_version(
    packet: &IncompatibleProtocolVersion,
) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_nack(buf: &[u8]) -> Result<Nack> { panic!("STUB: not implemented") }

pub fn write_packet_nack(packet: &Nack) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_ack(buf: &[u8]) -> Result<Ack> { panic!("STUB: not implemented") }

pub fn write_packet_ack(packet: &Ack) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_connection_request(buf: &[u8]) -> Result<ConnectionRequest> { panic!("STUB: not implemented") }

pub fn write_packet_connection_request(packet: &ConnectionRequest) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_connection_request_accepted(buf: &[u8]) -> Result<ConnectionRequestAccepted> { panic!("STUB: not implemented") }

pub fn write_packet_connection_request_accepted(
    packet: &ConnectionRequestAccepted,
) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_new_incomming_connection(buf: &[u8]) -> Result<NewIncomingConnection> { panic!("STUB: not implemented") }

pub fn write_packet_new_incomming_connection(packet: &NewIncomingConnection) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn read_packet_connected_ping(buf: &[u8]) -> Result<ConnectedPing> { panic!("STUB: not implemented") }

pub fn write_packet_connected_ping(packet: &ConnectedPing) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

pub fn _read_packet_connected_pong(buf: &[u8]) -> Result<ConnectedPong> { panic!("STUB: not implemented") }

pub fn write_packet_connected_pong(packet: &ConnectedPong) -> Result<Vec<u8>> { panic!("STUB: not implemented") }
