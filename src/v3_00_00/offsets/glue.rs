use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct GlueOffsets {
  pub runtime_space: Offset,
  pub ids: Offset,
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub art_mesh_indices_a: Offset,
  pub art_mesh_indices_b: Offset,
  pub glue_info_sources_begin_indices: Offset,
  pub glue_info_sources_counts: Offset,
}

impl From<[Offset; 9]> for GlueOffsets {
  fn from(
    [
      runtime_space,
      ids,
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      art_mesh_indices_a,
      art_mesh_indices_b,
      glue_info_sources_begin_indices,
      glue_info_sources_counts,
    ]: [Offset; 9],
  ) -> Self {
    Self {
      runtime_space,
      ids,
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      art_mesh_indices_a,
      art_mesh_indices_b,
      glue_info_sources_begin_indices,
      glue_info_sources_counts,
    }
  }
}

impl<T> Reading<GlueOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<GlueOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
