use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct PositionIndicesOffsets {
  pub indices: Offset,
}

impl From<[Offset; 1]> for PositionIndicesOffsets {
  fn from([indices]: [Offset; 1]) -> Self {
    Self { indices }
  }
}

impl<T> Reading<PositionIndicesOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<PositionIndicesOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
