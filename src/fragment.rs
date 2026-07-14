use crate::arq::{FrameSetPacket, Reliability};
use crate::error::*;
use std::collections::HashMap;

struct Fragment {
    pub flags: u8,
    pub compound_size: u32,
    pub ordered_frame_index: u32,
    pub frames: HashMap<u32, FrameSetPacket>,
}

impl Fragment {
    pub fn new(flags: u8, compound_size: u32, ordered_frame_index: u32) -> Self { panic!("STUB: not implemented") }

    pub fn full(&self) -> bool { panic!("STUB: not implemented") }

    pub fn insert(&mut self, frame: FrameSetPacket) { panic!("STUB: not implemented") }

    pub fn merge(&mut self) -> Result<FrameSetPacket> { panic!("STUB: not implemented") }
}

pub struct FragmentQ {
    fragments: HashMap<u16, Fragment>,
}

impl FragmentQ {
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn insert(&mut self, frame: FrameSetPacket) { panic!("STUB: not implemented") }

    pub fn flush(&mut self) -> Result<Vec<FrameSetPacket>> { panic!("STUB: not implemented") }

    pub fn size(&self) -> usize { panic!("STUB: not implemented") }
}
