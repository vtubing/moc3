use crate::prelude::*;

use byteorder::{BigEndian, LittleEndian};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub enum Reader {
  BigEndian(super::ReaderForByteOrder<BigEndian>),
  LittleEndian(super::ReaderForByteOrder<LittleEndian>),
}

impl TryFrom<Vec<u8>> for Reader {
  type Error = Error;

  fn try_from(moc3: Vec<u8>) -> std::result::Result<Self, Self::Error> {
    let mut cursor = Cursor::new(moc3);

    let header = {
      let mut buf = [0u8; 64];
      cursor.read_exact(&mut buf)?;
      Header::from(buf)
    };

    if !header.magic.is_valid() {
      return Err(Error::BadMagic);
    }

    cursor.rewind()?;

    let reader = if header.is_big_endian {
      Self::BigEndian(cursor.into())
    } else {
      Self::LittleEndian(cursor.into())
    };

    Ok(reader)
  }
}

impl Reader {
  pub fn read_u8s_to_new<const N: usize>(&mut self) -> Result<Self> {
    let cursor = Cursor::new(self.read::<N>()?.to_vec());
    let reader = match self {
      Self::BigEndian(_) => Self::BigEndian(cursor.into()),
      Self::LittleEndian(_) => Self::LittleEndian(cursor.into()),
    };
    Ok(reader)
  }
}

impl Seeking for Reader {
  fn seek(&mut self, pos: SeekFrom) -> Result<&mut Self> {
    match self {
      Self::BigEndian(reader) => {
        reader.seek(pos)?;
      }
      Self::LittleEndian(reader) => {
        reader.seek(pos)?;
      }
    }
    Ok(self)
  }

  fn stream_position(&mut self) -> Result<u64> {
    match self {
      Self::BigEndian(reader) => reader.stream_position(),
      Self::LittleEndian(reader) => reader.stream_position(),
    }
  }
}

impl Reading<u8> for Reader {
  fn read_one(&mut self) -> Result<u8> {
    match self {
      Self::BigEndian(reader) => reader.read_one(),
      Self::LittleEndian(reader) => reader.read_one(),
    }
  }

  fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
    match self {
      Self::BigEndian(reader) => reader.read::<COUNT>(),
      Self::LittleEndian(reader) => reader.read::<COUNT>(),
    }
  }
}

impl Reading<u32> for Reader {
  fn read_one(&mut self) -> Result<u32> {
    match self {
      Self::BigEndian(reader) => reader.read_one(),
      Self::LittleEndian(reader) => reader.read_one(),
    }
  }

  fn read<const COUNT: usize>(&mut self) -> Result<[u32; COUNT]> {
    match self {
      Self::BigEndian(reader) => reader.read::<COUNT>(),
      Self::LittleEndian(reader) => reader.read::<COUNT>(),
    }
  }
}

impl Reading<i16> for Reader {
  fn read_one(&mut self) -> Result<i16> {
    match self {
      Self::BigEndian(reader) => reader.read_one(),
      Self::LittleEndian(reader) => reader.read_one(),
    }
  }

  fn read<const COUNT: usize>(&mut self) -> Result<[i16; COUNT]> {
    match self {
      Self::BigEndian(reader) => reader.read::<COUNT>(),
      Self::LittleEndian(reader) => reader.read::<COUNT>(),
    }
  }
}

impl Reading<i32> for Reader {
  fn read_one(&mut self) -> Result<i32> {
    match self {
      Self::BigEndian(reader) => reader.read_one(),
      Self::LittleEndian(reader) => reader.read_one(),
    }
  }

  fn read<const COUNT: usize>(&mut self) -> Result<[i32; COUNT]> {
    match self {
      Self::BigEndian(reader) => reader.read::<COUNT>(),
      Self::LittleEndian(reader) => reader.read::<COUNT>(),
    }
  }
}

impl Reading<f32> for Reader {
  fn read_one(&mut self) -> Result<f32> {
    match self {
      Self::BigEndian(reader) => reader.read_one(),
      Self::LittleEndian(reader) => reader.read_one(),
    }
  }

  fn read<const COUNT: usize>(&mut self) -> Result<[f32; COUNT]> {
    match self {
      Self::BigEndian(reader) => reader.read::<COUNT>(),
      Self::LittleEndian(reader) => reader.read::<COUNT>(),
    }
  }
}
