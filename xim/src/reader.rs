#![allow(unused_variables)]

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("End of Stream")]
    EndOfStream,
    #[error("Invalid Data {0}: {1}")]
    InvalidData(&'static str, String),
}

pub type Result<T> = std::result::Result<T, ReadError>;

pub struct Reader<'a> {
    b: &'a [u8],
    len: usize,
}

impl<'a> Reader<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Self { b, len: 0 }
    }

    pub fn eos(&self) -> ReadError {
        ReadError::EndOfStream
    }

    pub fn invalid_data(&self, ty: &'static str, item: impl ToString) -> ReadError {
        ReadError::InvalidData(ty, item.to_string())
    }

    pub fn clear_len(&mut self) {
        self.len = 0;
    }

    pub fn bytes_len(&self) -> usize {
        self.b.len()
    }

    pub fn consume(&mut self, len: impl Into<usize>) -> Result<&'a [u8]> {
        let len = len.into();
        if self.b.len() >= len {
            let (out, new) = self.b.split_at(len);
            self.b = new;
            self.len += len;
            Ok(out)
        } else {
            Err(self.eos())
        }
    }

    pub fn u8(&mut self) -> Result<u8> {
        self.len += 1;
        match self.b {
            [b, other @ ..] => {
                self.b = other;
                Ok(*b)
            }
            [] => Err(self.eos()),
        }
    }

    pub fn u16(&mut self) -> Result<u16> {
        Readable::read(self)
    }

    pub fn pad(&mut self) {
        let p = (4 - (self.len % 4)) % 4;
        self.len = 0;
        self.b = &self.b[p..];
    }
}

pub trait Readable<'a>: Sized {
    fn read(reader: &mut Reader<'a>) -> Result<Self>;
}
