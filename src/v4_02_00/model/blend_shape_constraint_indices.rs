use super::offsets::BlendShapeConstraintIndicesOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeConstraintIndex {
  pub blend_shape_constraint_sources_index: i32,
}

impl ExtractFromOffsets for BlendShapeConstraintIndex {
  type Offsets = BlendShapeConstraintIndicesOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      blend_shape_constraint_sources_index: moc3.read_one_at_offset_with_index(offsets.blend_shape_constraint_sources_indices, index)?,
    })
  }
}
