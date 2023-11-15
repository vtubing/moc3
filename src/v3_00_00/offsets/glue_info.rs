use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct GlueInfoOffsets {
  pub weights: Offset,
  pub position_indices: Offset,
}

impl From<[Offset; 2]> for GlueInfoOffsets {
  fn from([weights, position_indices]: [Offset; 2]) -> Self {
    Self { weights, position_indices }
  }
}

impl<T> Reading<GlueInfoOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<GlueInfoOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
