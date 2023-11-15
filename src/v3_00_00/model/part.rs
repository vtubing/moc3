use super::offsets::PartOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Part {
  pub id: ID,
  pub keyform_binding_sources_index: i32,
  pub keyform_sources_begin_index: i32,
  pub keyform_sources_count: i32,
  pub is_visible: Bool32,
  pub is_enabled: Bool32,
  pub parent_part_index: i32,
}

impl ExtractFromOffsets for Part {
  type Offsets = PartOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      id: moc3.read_one_at_offset_with_index(offsets.ids, index)?,
      keyform_binding_sources_index: moc3.read_one_at_offset_with_index(offsets.keyform_binding_sources_indices, index)?,
      keyform_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_sources_begin_indices, index)?,
      keyform_sources_count: moc3.read_one_at_offset_with_index(offsets.keyform_sources_counts, index)?,
      is_visible: moc3.read_one_at_offset_with_index(offsets.is_visible, index)?,
      is_enabled: moc3.read_one_at_offset_with_index(offsets.is_enabled, index)?,
      parent_part_index: moc3.read_one_at_offset_with_index(offsets.parent_part_indices, index)?,
    })
  }
}
