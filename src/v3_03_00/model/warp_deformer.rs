use super::offsets::WarpDeformerOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WarpDeformer {
  pub keyform_binding_sources_index: i32,
  pub keyform_sources_begin_index: i32,
  pub keyform_sources_count: i32,
  pub vertex_count: i32,
  pub rows: u32,
  pub columns: u32,
  pub is_quad_source: Bool32,
}

impl ExtractFromOffsets for WarpDeformer {
  type Offsets = WarpDeformerOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      keyform_binding_sources_index: moc3.read_one_at_offset_with_index(offsets.keyform_binding_sources_indices, index)?,
      keyform_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_sources_begin_indices, index)?,
      keyform_sources_count: moc3.read_one_at_offset_with_index(offsets.keyform_sources_counts, index)?,
      vertex_count: moc3.read_one_at_offset_with_index(offsets.vertex_counts, index)?,
      rows: moc3.read_one_at_offset_with_index(offsets.rows, index)?,
      columns: moc3.read_one_at_offset_with_index(offsets.columns, index)?,
      is_quad_source: moc3.read_one_at_offset_with_index(offsets.is_quad_source, index)?,
    })
  }
}
