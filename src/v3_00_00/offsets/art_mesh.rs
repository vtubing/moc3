use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ArtMeshOffsets {
  pub runtime_space_0: Offset,
  pub runtime_space_1: Offset,
  pub runtime_space_2: Offset,
  pub runtime_space_3: Offset,
  pub ids: Offset,
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub is_visible: Offset,
  pub is_enabled: Offset,
  pub parent_part_indices: Offset,
  pub parent_deformer_indices: Offset,
  pub texture_nos: Offset,
  pub drawable_flags: Offset,
  pub vertex_counts: Offset,
  pub uv_sources_begin_indices: Offset,
  pub position_index_sources_begin_indices: Offset,
  pub position_index_sources_counts: Offset,
  pub drawable_mask_sources_begin_indices: Offset,
  pub drawable_mask_sources_counts: Offset,
}

impl From<[Offset; 20]> for ArtMeshOffsets {
  fn from(
    [
      runtime_space_0,
      runtime_space_1,
      runtime_space_2,
      runtime_space_3,
      ids,
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      is_visible,
      is_enabled,
      parent_part_indices,
      parent_deformer_indices,
      texture_nos,
      drawable_flags,
      vertex_counts,
      uv_sources_begin_indices,
      position_index_sources_begin_indices,
      position_index_sources_counts,
      drawable_mask_sources_begin_indices,
      drawable_mask_sources_counts,
    ]: [Offset;20],
  ) -> Self {
    Self {
      runtime_space_0,
      runtime_space_1,
      runtime_space_2,
      runtime_space_3,
      ids,
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      is_visible,
      is_enabled,
      parent_part_indices,
      parent_deformer_indices,
      texture_nos,
      drawable_flags,
      vertex_counts,
      uv_sources_begin_indices,
      position_index_sources_begin_indices,
      position_index_sources_counts,
      drawable_mask_sources_begin_indices,
      drawable_mask_sources_counts,
    }
  }
}

impl<T> Reading<ArtMeshOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<ArtMeshOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
