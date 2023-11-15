use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeParameterBindingOffsets {
  pub keys_sources_begin_indices: Offset,
  pub keys_sources_counts: Offset,
  pub base_key_indices: Offset,
}

impl From<[Offset; 3]> for BlendShapeParameterBindingOffsets {
  fn from([keys_sources_begin_indices, keys_sources_counts, base_key_indices]: [Offset; 3]) -> Self {
    Self {
      keys_sources_begin_indices,
      keys_sources_counts,
      base_key_indices,
    }
  }
}

impl<T> Reading<BlendShapeParameterBindingOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<BlendShapeParameterBindingOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
