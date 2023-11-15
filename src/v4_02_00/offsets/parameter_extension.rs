use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ParameterExtensionOffsets {
  pub runtime_space_0: Offset,
  pub keys_sources_begin_indices: Offset,
  pub keys_sources_counts: Offset,
}

impl From<[Offset; 3]> for ParameterExtensionOffsets {
  fn from([runtime_space_0, keys_sources_begin_indices, keys_sources_counts]: [Offset; 3]) -> Self {
    Self {
      runtime_space_0,
      keys_sources_begin_indices,
      keys_sources_counts,
    }
  }
}

impl<T> Reading<ParameterExtensionOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<ParameterExtensionOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
