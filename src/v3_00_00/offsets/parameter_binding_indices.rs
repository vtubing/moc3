use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ParameterBindingIndicesOffsets {
  pub binding_sources_indices: Offset,
}

impl From<[Offset; 1]> for ParameterBindingIndicesOffsets {
  fn from([binding_sources_indices]: [Offset; 1]) -> Self {
    Self { binding_sources_indices }
  }
}

impl<T> Reading<ParameterBindingIndicesOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<ParameterBindingIndicesOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
