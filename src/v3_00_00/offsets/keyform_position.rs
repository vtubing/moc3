use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct KeyformPositionOffsets {
  pub xys: Offset,
}

impl From<[Offset; 1]> for KeyformPositionOffsets {
  fn from([xys]: [Offset; 1]) -> Self {
    Self { xys }
  }
}

impl<T> Reading<KeyformPositionOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<KeyformPositionOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
