use super::offsets::BlendShapeKeyformBindingOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeKeyformBinding {
  pub parameter_binding_sources_index: i32,
  pub keyform_sources_blend_shape_index: i32,
  pub keyform_sources_blend_shape_count: i32,
  pub blend_shape_constraint_index_sources_being_index: i32,
  pub blend_shape_constraint_index_sources_count: i32,
}

impl ExtractFromOffsets for BlendShapeKeyformBinding {
  type Offsets = BlendShapeKeyformBindingOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      parameter_binding_sources_index: moc3.read_one_at_offset_with_index(offsets.parameter_binding_sources_indices, index)?,
      keyform_sources_blend_shape_index: moc3.read_one_at_offset_with_index(offsets.keyform_sources_blend_shape_indices, index)?,
      keyform_sources_blend_shape_count: moc3.read_one_at_offset_with_index(offsets.keyform_sources_blend_shape_counts, index)?,
      blend_shape_constraint_index_sources_being_index: moc3.read_one_at_offset_with_index(offsets.blend_shape_constraint_index_sources_begin_indices, index)?,
      blend_shape_constraint_index_sources_count: moc3.read_one_at_offset_with_index(offsets.blend_shape_constraint_index_sources_counts, index)?,
    })
  }
}
