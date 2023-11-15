use super::offsets::RotationDeformerOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct RotationDeformer {
  pub keyform_binding_sources_index: i32,
  pub keyform_sources_begin_index: i32,
  pub keyform_sources_count: i32,
  pub base_angles: f32,
}

impl ExtractFromOffsets for RotationDeformer {
  type Offsets = RotationDeformerOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      keyform_binding_sources_index: moc3.read_one_at_offset_with_index(offsets.keyform_binding_sources_indices, index)?,
      keyform_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_sources_begin_indices, index)?,
      keyform_sources_count: moc3.read_one_at_offset_with_index(offsets.keyform_sources_counts, index)?,
      base_angles: moc3.read_one_at_offset_with_index(offsets.base_angles, index)?,
    })
  }
}
