use super::offsets::GlueOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Glue {
  pub id: ID,
  pub keyform_binding_sources_index: i32,
  pub keyform_sources_begin_index: i32,
  pub keyform_sources_count: i32,
  pub art_mesh_index_a: i32,
  pub art_mesh_index_b: i32,
  pub glue_info_sources_begin_index: i32,
  pub glue_info_sources_count: i32,
}

impl ExtractFromOffsets for Glue {
  type Offsets = GlueOffsets;
  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      id: moc3.read_one_at_offset_with_index(offsets.ids, index)?,
      keyform_binding_sources_index: moc3.read_one_at_offset_with_index(offsets.keyform_binding_sources_indices, index)?,
      keyform_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_sources_begin_indices, index)?,
      keyform_sources_count: moc3.read_one_at_offset_with_index(offsets.keyform_sources_counts, index)?,
      art_mesh_index_a: moc3.read_one_at_offset_with_index(offsets.art_mesh_indices_a, index)?,
      art_mesh_index_b: moc3.read_one_at_offset_with_index(offsets.art_mesh_indices_b, index)?,
      glue_info_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.glue_info_sources_begin_indices, index)?,
      glue_info_sources_count: moc3.read_one_at_offset_with_index(offsets.glue_info_sources_counts, index)?,
    })
  }
}
