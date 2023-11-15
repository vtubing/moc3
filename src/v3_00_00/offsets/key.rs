use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct KeyOffsets {
  pub values: Offset,
}

impl From<[Offset; 1]> for KeyOffsets {
  fn from([values]: [Offset; 1]) -> Self {
    Self { values }
  }
}

impl<T> Reading<KeyOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<KeyOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
