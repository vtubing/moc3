use super::offsets::KeyOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Key {
  pub value: f32,
}

impl ExtractFromOffsets for Key {
  type Offsets = KeyOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      value: moc3.read_one_at_offset_with_index(offsets.values, index)?,
    })
  }
}
