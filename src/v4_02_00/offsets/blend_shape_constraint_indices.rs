use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeConstraintIndicesOffsets {
  pub blend_shape_constraint_sources_indices: Offset,
}

impl From<[Offset; 1]> for BlendShapeConstraintIndicesOffsets {
  fn from([blend_shape_constraint_sources_indices]: [Offset; 1]) -> Self {
    Self {
      blend_shape_constraint_sources_indices,
    }
  }
}

impl<T> Reading<BlendShapeConstraintIndicesOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<BlendShapeConstraintIndicesOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
