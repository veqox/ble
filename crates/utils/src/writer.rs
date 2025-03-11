use core::{error::Error, fmt::Display};

use crate::slice;

#[derive(Debug)]
pub enum WriteError {
    BufferOverflow,
    InvalidFormat,
}

impl Display for WriteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferOverflow => write!(f, "BufferOverflow"),
            Self::InvalidFormat => write!(f, "WriteError"),
        }
    }
}

impl Error for WriteError {}

#[derive(Debug)]
pub struct Writer<'p> {
    buf: &'p mut [u8],
    pub pos: usize,
}

impl<'p> Writer<'p> {
    pub fn new(buf: &'p mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), WriteError> {
        self.write_u8_slice(&value.to_le_bytes())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), WriteError> {
        self.write_u8_slice(&value.to_le_bytes())
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), WriteError> {
        self.write_u8_slice(&value.to_le_bytes())
    }

    pub fn write_u64(&mut self, value: u64) -> Result<(), WriteError> {
        self.write_u8_slice(&value.to_le_bytes())
    }

    pub fn write_u128(&mut self, value: u128) -> Result<(), WriteError> {
        self.write_u8_slice(&value.to_le_bytes())
    }

    pub fn write_u8_slice(&mut self, slice: &[u8]) -> Result<(), WriteError> {
        if self.pos + slice.len() >= self.buf.len() {
            return Err(WriteError::BufferOverflow);
        }

        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();

        Ok(())
    }

    pub fn write_u16_slice(&mut self, slice: &[u16]) -> Result<(), WriteError> {
        if self.pos + slice.len() >= self.buf.len() {
            return Err(WriteError::BufferOverflow);
        }

        let slice = match slice::as_u8_slice(slice) {
            Some(slice) => slice,
            None => return Err(WriteError::BufferOverflow),
        };

        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();

        Ok(())
    }

    pub fn write_u32_slice(&mut self, slice: &[u32]) -> Result<(), WriteError> {
        if self.pos + slice.len() >= self.buf.len() {
            return Err(WriteError::BufferOverflow);
        }

        let slice = match slice::as_u8_slice(slice) {
            Some(slice) => slice,
            None => return Err(WriteError::BufferOverflow),
        };

        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();

        Ok(())
    }

    pub fn write_u64_slice(&mut self, slice: &[u64]) -> Result<(), WriteError> {
        if self.pos + slice.len() >= self.buf.len() {
            return Err(WriteError::BufferOverflow);
        }

        let slice = match slice::as_u8_slice(slice) {
            Some(slice) => slice,
            None => return Err(WriteError::BufferOverflow),
        };

        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();

        Ok(())
    }

    pub fn write_u128_slice(&mut self, slice: &[u128]) -> Result<(), WriteError> {
        if self.pos + slice.len() >= self.buf.len() {
            return Err(WriteError::BufferOverflow);
        }

        let slice = match slice::as_u8_slice(slice) {
            Some(slice) => slice,
            None => return Err(WriteError::BufferOverflow),
        };

        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();

        Ok(())
    }
}
