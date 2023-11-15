use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct KeyformColorOffsets {
  pub r: Offset,
  pub g: Offset,
  pub b: Offset,
}

impl From<[Offset; 3]> for KeyformColorOffsets {
  fn from([r, g, b]: [Offset; 3]) -> Self {
    Self { r, g, b }
  }
}

impl<T> Reading<KeyformColorOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<KeyformColorOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
