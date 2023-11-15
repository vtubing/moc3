use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ParameterBindingOffsets {
  pub keys_sources_begin_indices: Offset,
  pub keys_sources_counts: Offset,
}

impl From<[Offset; 2]> for ParameterBindingOffsets {
  fn from([keys_sources_begin_indices, keys_sources_counts]: [Offset; 2]) -> Self {
    Self {
      keys_sources_begin_indices,
      keys_sources_counts,
    }
  }
}

impl<T> Reading<ParameterBindingOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<ParameterBindingOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
