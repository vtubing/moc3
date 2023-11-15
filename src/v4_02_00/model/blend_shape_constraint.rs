use super::offsets::BlendShapeConstraintOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeConstraint {
  pub parameter_index: i32,
  pub blend_shape_constraint_value_sources_begin_index: i32,
  pub blend_shape_constraint_value_sources_count: i32,
}

impl ExtractFromOffsets for BlendShapeConstraint {
  type Offsets = BlendShapeConstraintOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      parameter_index: moc3.read_one_at_offset_with_index(offsets.parameter_indices, index)?,
      blend_shape_constraint_value_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.blend_shape_constraint_value_sources_begin_indices, index)?,
      blend_shape_constraint_value_sources_count: moc3.read_one_at_offset_with_index(offsets.blend_shape_constraint_value_sources_counts, index)?,
    })
  }
}
