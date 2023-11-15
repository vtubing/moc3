use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ParameterOffsets {
  pub runtime_space_0: Offset,
  pub ids: Offset,
  pub max_values: Offset,
  pub min_values: Offset,
  pub default_values: Offset,
  pub is_repeat: Offset,
  pub decimal_places: Offset,
  pub parameter_binding_sources_begin_indices: Offset,
  pub parameter_binding_sources_counts: Offset,
}

impl From<[Offset; 9]> for ParameterOffsets {
  fn from([runtime_space_0, ids, max_values, min_values, default_values, is_repeat, decimal_places, parameter_binding_sources_begin_indices, parameter_binding_sources_counts]: [Offset; 9]) -> Self {
    Self {
      runtime_space_0,
      ids,
      max_values,
      min_values,
      default_values,
      is_repeat,
      decimal_places,
      parameter_binding_sources_begin_indices,
      parameter_binding_sources_counts,
    }
  }
}

impl<T> Reading<ParameterOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<ParameterOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
