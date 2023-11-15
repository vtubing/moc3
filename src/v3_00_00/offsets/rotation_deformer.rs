use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct RotationDeformerOffsets {
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub base_angles: Offset,
}

impl From<[Offset; 4]> for RotationDeformerOffsets {
  fn from([keyform_binding_sources_indices, keyform_sources_begin_indices, keyform_sources_counts, base_angles]: [Offset; 4]) -> Self {
    Self {
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      base_angles,
    }
  }
}

impl<T> Reading<RotationDeformerOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<RotationDeformerOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
