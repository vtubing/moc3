use crate::{Reading, Result, Seeking};
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::marker::PhantomData;

pub struct ReaderForByteOrder<T>
where
  T: ByteOrder,
{
  cursor: Cursor<Vec<u8>>,
  _endian: PhantomData<T>,
}

impl<T> From<Cursor<Vec<u8>>> for ReaderForByteOrder<T>
where
  T: ByteOrder,
{
  fn from(cursor: Cursor<Vec<u8>>) -> Self {
    Self { cursor, _endian: PhantomData }
  }
}

impl<T> From<Vec<u8>> for ReaderForByteOrder<T>
where
  T: ByteOrder,
{
  fn from(inner: Vec<u8>) -> Self {
    Cursor::new(inner).into()
  }
}

impl<T: ByteOrder> Seeking for ReaderForByteOrder<T> {
  fn seek(&mut self, pos: SeekFrom) -> Result<&mut Self> {
    self.cursor.seek(pos)?;
    Ok(self)
  }

  fn stream_position(&mut self) -> Result<u64> {
    Ok(self.cursor.stream_position()?)
  }
}

impl<T: ByteOrder> Reading<u8> for ReaderForByteOrder<T> {
  fn read_one(&mut self) -> Result<u8> {
    Ok(self.cursor.read_u8()?)
  }

  fn read<const N: usize>(&mut self) -> Result<[u8; N]> {
    let mut values = [0u8; N];
    self.cursor.read_exact(&mut values)?;
    Ok(values)
  }
}

impl<T: ByteOrder> Reading<u32> for ReaderForByteOrder<T> {
  fn read_one(&mut self) -> Result<u32> {
    Ok(self.cursor.read_u32::<T>()?)
  }

  fn read<const N: usize>(&mut self) -> Result<[u32; N]> {
    let mut values = [0u32; N];
    self.cursor.read_u32_into::<T>(&mut values)?;
    Ok(values)
  }
}

impl<T: ByteOrder> Reading<i16> for ReaderForByteOrder<T> {
  fn read_one(&mut self) -> Result<i16> {
    Ok(self.cursor.read_i16::<T>()?)
  }

  fn read<const N: usize>(&mut self) -> Result<[i16; N]> {
    let mut values = [0i16; N];
    self.cursor.read_i16_into::<T>(&mut values)?;
    Ok(values)
  }
}

impl<T: ByteOrder> Reading<i32> for ReaderForByteOrder<T> {
  fn read_one(&mut self) -> Result<i32> {
    Ok(self.cursor.read_i32::<T>()?)
  }

  fn read<const N: usize>(&mut self) -> Result<[i32; N]> {
    let mut values = [0i32; N];
    self.cursor.read_i32_into::<T>(&mut values)?;
    Ok(values)
  }
}

impl<T: ByteOrder> Reading<f32> for ReaderForByteOrder<T> {
  fn read_one(&mut self) -> Result<f32> {
    Ok(self.cursor.read_f32::<T>()?)
  }

  fn read<const N: usize>(&mut self) -> Result<[f32; N]> {
    let mut values = [0f32; N];
    self.cursor.read_f32_into::<T>(&mut values)?;
    Ok(values)
  }
}

impl From<ReaderForByteOrder<BigEndian>> for crate::Reader {
  fn from(reader: ReaderForByteOrder<BigEndian>) -> Self {
    Self::BigEndian(reader)
  }
}

impl From<ReaderForByteOrder<LittleEndian>> for crate::Reader {
  fn from(reader: ReaderForByteOrder<LittleEndian>) -> Self {
    Self::LittleEndian(reader)
  }
}
