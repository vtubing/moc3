use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct UvOffsets {
  pub uvs: Offset,
}

impl From<[Offset; 1]> for UvOffsets {
  fn from([uvs]: [Offset; 1]) -> Self {
    Self { uvs }
  }
}

impl<T> Reading<UvOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<UvOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
