use crate::error::*;
use crate::utils::Endian;
use bytes::{Buf, BufMut};
use std::{
    io::{Cursor, Read},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    str,
};

#[derive(Clone)]
pub struct RaknetWriter {
    buf: Vec<u8>,
}

impl RaknetWriter {
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn write(&mut self, v: &[u8]) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_u8(&mut self, v: u8) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_i16(&mut self, v: i16, n: Endian) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_u16(&mut self, v: u16, n: Endian) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_u24(&mut self, v: u32, n: Endian) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_u32(&mut self, v: u32, n: Endian) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_i32(&mut self, v: i32, n: Endian) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_i64(&mut self, v: i64, n: Endian) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_magic(&mut self) -> Result<usize> { panic!("STUB: not implemented") }

    pub fn write_u64(&mut self, v: u64, n: Endian) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_string(&mut self, body: &str) -> Result<()> { panic!("STUB: not implemented") }

    pub fn write_address(&mut self, address: SocketAddr) -> Result<()> { panic!("STUB: not implemented") }

    pub fn get_raw_payload(self) -> Vec<u8> { panic!("STUB: not implemented") }

    pub fn _pos(&self) -> u64 { panic!("STUB: not implemented") }
}

pub struct RaknetReader {
    buf: Cursor<Vec<u8>>,
}

impl RaknetReader {
    pub fn new(buf: Vec<u8>) -> Self { panic!("STUB: not implemented") }
    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> { panic!("STUB: not implemented") }
    pub fn read_u8(&mut self) -> Result<u8> { panic!("STUB: not implemented") }

    pub fn read_u16(&mut self, n: Endian) -> Result<u16> { panic!("STUB: not implemented") }

    pub fn read_u24(&mut self, n: Endian) -> Result<u32> { panic!("STUB: not implemented") }

    pub fn read_u32(&mut self, n: Endian) -> Result<u32> { panic!("STUB: not implemented") }

    pub fn read_u64(&mut self, n: Endian) -> Result<u64> { panic!("STUB: not implemented") }
    pub fn read_i64(&mut self, n: Endian) -> Result<i64> { panic!("STUB: not implemented") }

    pub fn read_string(&mut self) -> Result<String> { panic!("STUB: not implemented") }

    pub fn read_magic(&mut self) -> Result<bool> { panic!("STUB: not implemented") }

    pub fn read_address(&mut self) -> Result<SocketAddr> { panic!("STUB: not implemented") }

    pub fn next(&mut self, n: u64) { panic!("STUB: not implemented") }

    pub fn pos(&self) -> u64 { panic!("STUB: not implemented") }
}

#[tokio::test]
async fn test_u24_encode_decode() {
    let a: u32 = 65535 * 21;
    let b = a.to_le_bytes();
    let mut reader = RaknetReader::new(b.to_vec());

    let c = reader.read_u24(Endian::Little).unwrap();

    assert!(a == c);

    let mut writer = RaknetWriter::new();
    writer.write_u24(a, Endian::Little).unwrap();

    let buf = writer.get_raw_payload();
    let mut reader = RaknetReader::new(buf);

    let c = reader.read_u24(Endian::Little).unwrap();

    assert!(a == c);
}
