use crate::prelude::*;

use std::io::SeekFrom;

pub(crate) trait Seeking {
  fn seek(&mut self, pos: SeekFrom) -> Result<&mut Self>;

  fn seek_to(&mut self, offset: Offset) -> Result<&mut Self> {
    self.seek(SeekFrom::Start(*offset))
  }

  fn seek_to_index_from_offset_by_size(&mut self, offset: Offset, index: u64, size: u64) -> Result<&mut Self> {
    self.seek_to(Offset(*offset + (index * size)))
  }

  fn stream_position(&mut self) -> Result<u64>;

  fn rewind(&mut self) -> Result<&mut Self> {
    self.seek(SeekFrom::Start(0))
  }

  fn len(&mut self) -> Result<u64> {
    let initial = Offset(self.stream_position()?);
    self.seek(SeekFrom::End(0))?;
    let len = self.stream_position()?;
    self.seek_to(initial)?;
    Ok(len)
  }
}

pub(crate) trait Reading<T: Default + std::fmt::Debug>: Seeking {
  const SIZE: usize = std::mem::size_of::<T>();

  fn read_one(&mut self) -> Result<T>;

  fn read_one_at_offset(&mut self, offset: Offset) -> Result<T> {
    self.seek_to(offset).and_then(Self::read_one)
  }

  fn read_one_at_offset_with_index(&mut self, offset: Offset, index: u64) -> Result<T> {
    let value = self.seek_to_index_from_offset_by_size(offset, index, Self::SIZE.try_into()?).and_then(Self::read_one)?;
    if index < 10 {
      debug!("read_one_at_offset_with_index(offset={offset:?}, index={index}) -> {value:?}");
    }
    Ok(value)
  }

  fn read<const COUNT: usize>(&mut self) -> Result<[T; COUNT]> {
    let mut values = [(); COUNT].map(|_| T::default());
    for n in 0..COUNT {
      values[n] = self.read_one()?;
    }
    Ok(values)
  }

  fn read_at_offset<const COUNT: usize>(&mut self, offset: Offset) -> Result<[T; COUNT]> {
    self.seek_to(offset).and_then(Self::read::<COUNT>)
  }

  fn read_at_offset_with_index<const COUNT: usize>(&mut self, offset: Offset, index: u64) -> Result<[T; COUNT]> {
    let values = self.seek_to_index_from_offset_by_size(offset, index, Self::SIZE.try_into()?).and_then(Self::read::<COUNT>)?;
    if index < 10 {
      debug!("read_at_offset_with_index(offset={offset:?}, index={index}) -> {values:?}");
    }
    Ok(values)
  }

  fn read_at_offsets<const OFFSETS: usize>(&mut self, offsets: [Offset; OFFSETS]) -> Result<[T; OFFSETS]> {
    let mut values = [(); OFFSETS].map(|_| T::default());
    for n in 0..OFFSETS {
      values[n] = self.read_one_at_offset(offsets[n])?;
    }
    Ok(values)
  }

  fn read_at_offsets_with_index<const OFFSETS: usize>(&mut self, offsets: [Offset; OFFSETS], index: u64) -> Result<[T; OFFSETS]> {
    let mut values = [(); OFFSETS].map(|_| T::default());
    for n in 0..OFFSETS {
      values[n] = self.read_one_at_offset_with_index(offsets[n], index)?;
    }
    if index < 10 {
      debug!("read_at_offsets_with_index(offsets={offsets:?}, index={index}) -> {values:?}");
    }
    Ok(values)
  }
}
