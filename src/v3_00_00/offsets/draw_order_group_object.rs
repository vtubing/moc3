use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct DrawOrderGroupObjectOffsets {
  pub types: Offset,
  pub indices: Offset,
  pub self_indices: Offset,
}

impl From<[Offset; 3]> for DrawOrderGroupObjectOffsets {
  fn from([types, indices, self_indices]: [Offset; 3]) -> Self {
    Self { types, indices, self_indices }
  }
}

impl<T> Reading<DrawOrderGroupObjectOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<DrawOrderGroupObjectOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
