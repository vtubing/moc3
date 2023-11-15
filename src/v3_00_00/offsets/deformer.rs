use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct DeformerOffsets {
  pub runtime_space_0: Offset,
  pub ids: Offset,
  pub keyform_binding_sources_indices: Offset,
  pub is_visible: Offset,
  pub is_enabled: Offset,
  pub parent_part_indices: Offset,
  pub parent_deformer_indices: Offset,
  pub types: Offset,
  pub specific_sources_indices: Offset,
}

impl From<[Offset; 9]> for DeformerOffsets {
  fn from([runtime_space_0, ids, keyform_binding_sources_indices, is_visible, is_enabled, parent_part_indices, parent_deformer_indices, types, specific_sources_indices]: [Offset; 9]) -> Self {
    Self {
      runtime_space_0,
      ids,
      keyform_binding_sources_indices,
      is_visible,
      is_enabled,
      parent_part_indices,
      parent_deformer_indices,
      types,
      specific_sources_indices,
    }
  }
}

impl<T> Reading<DeformerOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<DeformerOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
