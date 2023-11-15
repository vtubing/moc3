use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeConstraintOffsets {
  pub parameter_indices: Offset,
  pub blend_shape_constraint_value_sources_begin_indices: Offset,
  pub blend_shape_constraint_value_sources_counts: Offset,
}

impl From<[Offset; 3]> for BlendShapeConstraintOffsets {
  fn from([parameter_indices, blend_shape_constraint_value_sources_begin_indices, blend_shape_constraint_value_sources_counts]: [Offset; 3]) -> Self {
    Self {
      parameter_indices,
      blend_shape_constraint_value_sources_begin_indices,
      blend_shape_constraint_value_sources_counts,
    }
  }
}

impl<T> Reading<BlendShapeConstraintOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<BlendShapeConstraintOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
