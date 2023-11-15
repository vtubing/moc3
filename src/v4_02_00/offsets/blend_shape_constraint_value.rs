use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeConstraintValueOffsets {
  pub keys: Offset,
  pub weights: Offset,
}

impl From<[Offset; 2]> for BlendShapeConstraintValueOffsets {
  fn from([keys, weights]: [Offset; 2]) -> Self {
    Self { keys, weights }
  }
}

impl<T> Reading<BlendShapeConstraintValueOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<BlendShapeConstraintValueOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
