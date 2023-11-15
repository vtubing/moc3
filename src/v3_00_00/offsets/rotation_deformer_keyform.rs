use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct RotationDeformerKeyformOffsets {
  pub opacities: Offset,
  pub angles: Offset,
  pub origin_x: Offset,
  pub origin_y: Offset,
  pub scales: Offset,
  pub is_reflect_x: Offset,
  pub is_reflect_y: Offset,
}

impl From<[Offset; 7]> for RotationDeformerKeyformOffsets {
  fn from([opacities, angles, origin_x, origin_y, scales, is_reflect_x, is_reflect_y]: [Offset; 7]) -> Self {
    Self {
      opacities,
      angles,
      origin_x,
      origin_y,
      scales,
      is_reflect_x,
      is_reflect_y,
    }
  }
}

impl<T> Reading<RotationDeformerKeyformOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<RotationDeformerKeyformOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
