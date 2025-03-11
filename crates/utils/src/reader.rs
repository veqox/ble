use core::{error::Error, fmt::Display};

use crate::slice;

#[derive(Debug)]
pub enum ReadError {
    BufferOverflow,
}

impl Display for ReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferOverflow => write!(f, "BufferOverflow"),
        }
    }
}

impl Error for ReadError {}

#[derive(Debug)]
pub struct Reader<'p> {
    buf: &'p [u8],
    pub pos: usize,
}

impl<'p> Reader<'p> {
    pub fn new(buf: &'p [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        Some(u8::from_le_bytes(
            self.read_u8_slice(size_of::<u8>())?.try_into().ok()?,
        ))
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(
            self.read_u8_slice(size_of::<u16>())?.try_into().ok()?,
        ))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(
            self.read_u8_slice(size_of::<u32>())?.try_into().ok()?,
        ))
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(
            self.read_u8_slice(size_of::<u64>())?.try_into().ok()?,
        ))
    }

    pub fn read_u128(&mut self) -> Option<u128> {
        Some(u128::from_le_bytes(
            self.read_u8_slice(size_of::<u128>())?.try_into().ok()?,
        ))
    }

    pub fn read_u8_slice(&mut self, len: usize) -> Option<&'p [u8]> {
        if self.remaining() < len {
            return None;
        }

        let slice = &self.buf[self.pos..(self.pos + len)];

        self.pos += len;
        Some(slice)
    }

    pub fn read_u16_slice(&mut self, len: usize) -> Option<&'p [u16]> {
        if self.remaining() < len {
            return None;
        }

        let slice = &self.buf[self.pos..(self.pos + len)];
        let slice = slice::as_u16_slice(slice)?;

        self.pos += len;
        Some(slice)
    }

    pub fn read_u32_slice(&mut self, len: usize) -> Option<&'p [u32]> {
        if self.remaining() < len {
            return None;
        }

        let slice = &self.buf[self.pos..(self.pos + len)];
        let slice = slice::as_u32_slice(slice)?;

        self.pos += len;
        Some(slice)
    }

    pub fn read_u64_slice(&mut self, len: usize) -> Option<&'p [u64]> {
        if self.remaining() < len {
            return None;
        }

        let slice = &self.buf[self.pos..(self.pos + len)];
        let slice = slice::as_u64_slice(slice)?;

        self.pos += len;
        Some(slice)
    }

    pub fn read_u128_slice(&mut self, len: usize) -> Option<&'p [u128]> {
        if self.remaining() < len {
            return None;
        }

        let slice = &self.buf[self.pos..(self.pos + len)];
        let slice = slice::as_u128_slice(slice)?;

        self.pos += len;
        Some(slice)
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), ReadError> {
        if pos > self.buf.len() {
            return Err(ReadError::BufferOverflow);
        }

        self.pos = pos;

        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }
}
