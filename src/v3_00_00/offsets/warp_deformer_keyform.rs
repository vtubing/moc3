use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WarpDeformerKeyformOffsets {
  pub opacities: Offset,
  pub keyform_position_sources_begin_indices: Offset,
}

impl From<[Offset; 2]> for WarpDeformerKeyformOffsets {
  fn from([opacities, keyform_position_sources_begin_indices]: [Offset; 2]) -> Self {
    Self {
      opacities,
      keyform_position_sources_begin_indices,
    }
  }
}

impl<T> Reading<WarpDeformerKeyformOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<WarpDeformerKeyformOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
