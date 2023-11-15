use super::offsets::BlendShapeOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShape {
  pub target_index: i32,
  pub blend_shape_keyform_binding_sources_begin_index: i32,
  pub blend_shape_keyform_binding_sources_count: i32,
}

impl ExtractFromOffsets for BlendShape {
  type Offsets = BlendShapeOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      target_index: moc3.read_one_at_offset_with_index(offsets.target_indices, index)?,
      blend_shape_keyform_binding_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.blend_shape_keyform_binding_sources_begin_indices, index)?,
      blend_shape_keyform_binding_sources_count: moc3.read_one_at_offset_with_index(offsets.blend_shape_keyform_binding_sources_counts, index)?,
    })
  }
}
