use super::offsets::PositionIndicesOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct PositionIndices {
  pub index: i16,
}

impl ExtractFromOffsets for PositionIndices {
  type Offsets = PositionIndicesOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      index: moc3.read_one_at_offset_with_index(offsets.indices, index)?,
    })
  }
}
