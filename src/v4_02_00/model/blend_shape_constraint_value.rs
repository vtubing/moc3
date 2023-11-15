use super::offsets::BlendShapeConstraintValueOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeConstraintValue {
  pub key: f32,
  pub weight: f32,
}

impl ExtractFromOffsets for BlendShapeConstraintValue {
  type Offsets = BlendShapeConstraintValueOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      key: moc3.read_one_at_offset_with_index(offsets.keys, index)?,
      weight: moc3.read_one_at_offset_with_index(offsets.weights, index)?,
    })
  }
}
