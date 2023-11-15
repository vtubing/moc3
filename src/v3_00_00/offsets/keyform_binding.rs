use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct KeyformBindingOffsets {
  pub parameter_binding_index_sources_begin_indices: Offset,
  pub parameter_binding_index_sources_counts: Offset,
}

impl From<[Offset; 2]> for KeyformBindingOffsets {
  fn from([parameter_binding_index_sources_begin_indices, parameter_binding_index_sources_counts]: [Offset; 2]) -> Self {
    Self {
      parameter_binding_index_sources_begin_indices,
      parameter_binding_index_sources_counts,
    }
  }
}

impl<T> Reading<KeyformBindingOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<KeyformBindingOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
