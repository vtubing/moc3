use super::offsets::KeyformColorOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct KeyformColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl ExtractFromOffsets for KeyformColor {
  type Offsets = KeyformColorOffsets;
  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      r: moc3.read_one_at_offset_with_index(offsets.r, index)?,
      g: moc3.read_one_at_offset_with_index(offsets.g, index)?,
      b: moc3.read_one_at_offset_with_index(offsets.b, index)?,
    })
  }
}
