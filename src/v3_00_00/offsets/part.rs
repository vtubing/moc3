use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct PartOffsets {
  pub runtime_space_0: Offset,
  pub ids: Offset,
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub is_visible: Offset,
  pub is_enabled: Offset,
  pub parent_part_indices: Offset,
}

impl From<[Offset; 8]> for PartOffsets {
  fn from([runtime_space_0, ids, keyform_binding_sources_indices, keyform_sources_begin_indices, keyform_sources_counts, is_visible, is_enabled, parent_part_indices]: [Offset; 8]) -> Self {
    Self {
      runtime_space_0,
      ids,
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      is_visible,
      is_enabled,
      parent_part_indices,
    }
  }
}

impl<T> Reading<PartOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<PartOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
