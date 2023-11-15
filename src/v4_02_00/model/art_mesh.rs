use super::offsets::ArtMeshOffsets;
use super::v4_00_00::model::ArtMeshDrawableFlags;
use crate::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct ArtMesh {
  pub id: ID,
  pub keyform_binding_sources_index: i32,
  pub keyform_sources_begin_index: i32,
  pub keyform_sources_count: i32,
  pub visible: Bool32,
  pub enabled: Bool32,
  pub parent_part_index: i32,
  pub parent_deformer_index: i32,
  pub texture_nos: u32,
  pub drawable_flags: ArtMeshDrawableFlags,
  pub vertex_counts: i32,
  pub uv_sources_begin_index: i32,
  pub position_index_sources_begin_index: i32,
  pub position_index_sources_count: i32,
  pub drawable_mask_sources_begin_index: i32,
  pub drawable_mask_sources_count: i32,
  pub keyform_color_sources_begin_index: i32,
}

impl ExtractFromOffsets for ArtMesh {
  type Offsets = ArtMeshOffsets;
  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      id: moc3.read_one_at_offset_with_index(offsets.ids, index)?,
      keyform_binding_sources_index: moc3.read_one_at_offset_with_index(offsets.keyform_binding_sources_indices, index)?,
      keyform_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_sources_begin_indices, index)?,
      keyform_sources_count: moc3.read_one_at_offset_with_index(offsets.keyform_sources_counts, index)?,
      visible: moc3.read_one_at_offset_with_index(offsets.is_visible, index)?,
      enabled: moc3.read_one_at_offset_with_index(offsets.is_enabled, index)?,
      parent_part_index: moc3.read_one_at_offset_with_index(offsets.parent_part_indices, index)?,
      parent_deformer_index: moc3.read_one_at_offset_with_index(offsets.parent_deformer_indices, index)?,
      texture_nos: moc3.read_one_at_offset_with_index(offsets.texture_nos, index)?,
      drawable_flags: moc3.read_one_at_offset_with_index(offsets.drawable_flags, index)?,
      vertex_counts: moc3.read_one_at_offset_with_index(offsets.vertex_counts, index)?,
      uv_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.uv_sources_begin_indices, index)?,
      position_index_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.position_index_sources_begin_indices, index)?,
      position_index_sources_count: moc3.read_one_at_offset_with_index(offsets.position_index_sources_counts, index)?,
      drawable_mask_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.drawable_mask_sources_begin_indices, index)?,
      drawable_mask_sources_count: moc3.read_one_at_offset_with_index(offsets.drawable_mask_sources_counts, index)?,
      keyform_color_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_color_sources_begin_indices, index)?,
    })
  }
}
