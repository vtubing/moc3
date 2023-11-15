use super::offsets::GlueInfoOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct GlueInfo {
  pub weight: f32,
  pub position_index: i16,
}

impl ExtractFromOffsets for GlueInfo {
  type Offsets = GlueInfoOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      weight: moc3.read_one_at_offset_with_index(offsets.weights, index)?,
      position_index: moc3.read_one_at_offset_with_index(offsets.position_indices, index)?,
    })
  }
}
