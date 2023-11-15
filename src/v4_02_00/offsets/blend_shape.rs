use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeOffsets {
  pub target_indices: Offset,
  pub blend_shape_keyform_binding_sources_begin_indices: Offset,
  pub blend_shape_keyform_binding_sources_counts: Offset,
}

impl From<[Offset; 3]> for BlendShapeOffsets {
  fn from([target_indices, blend_shape_keyform_binding_sources_begin_indices, blend_shape_keyform_binding_sources_counts]: [Offset; 3]) -> Self {
    Self {
      target_indices,
      blend_shape_keyform_binding_sources_begin_indices,
      blend_shape_keyform_binding_sources_counts,
    }
  }
}

impl<T> Reading<BlendShapeOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<BlendShapeOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
