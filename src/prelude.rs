#![allow(unused)]

pub(crate) use crate::error::Error;
pub(crate) use crate::header::Header;
pub(crate) use crate::reader::{Reader, Reading, Seeking};
pub(crate) use crate::types::*;
pub(crate) use crate::Result;
pub(crate) use log::{debug, error, info, trace, warn};
pub(crate) use std::io::{Cursor, SeekFrom};

pub trait ExtractFromOffsets: Sized {
  type Offsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self>;

  fn extract(count: u32, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Vec<Self>> {
    let mut values = Vec::new();

    for index in 0..count {
      let value = Self::extract_one(index.into(), offsets, moc3)?;
      values.push(value);
    }

    Ok(values)
  }
}
