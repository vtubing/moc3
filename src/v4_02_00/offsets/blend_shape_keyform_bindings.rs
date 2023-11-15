use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BlendShapeKeyformBindingOffsets {
  pub parameter_binding_sources_indices: Offset,
  pub keyform_sources_blend_shape_indices: Offset,
  pub keyform_sources_blend_shape_counts: Offset,
  pub blend_shape_constraint_index_sources_begin_indices: Offset,
  pub blend_shape_constraint_index_sources_counts: Offset,
}

impl From<[Offset; 5]> for BlendShapeKeyformBindingOffsets {
  fn from(
    [
      parameter_binding_sources_indices,
      keyform_sources_blend_shape_indices,
      keyform_sources_blend_shape_counts,
      blend_shape_constraint_index_sources_begin_indices,
      blend_shape_constraint_index_sources_counts,
    ]: [Offset;5],
  ) -> Self {
    Self {
      parameter_binding_sources_indices,
      keyform_sources_blend_shape_indices,
      keyform_sources_blend_shape_counts,
      blend_shape_constraint_index_sources_begin_indices,
      blend_shape_constraint_index_sources_counts,
    }
  }
}

impl<T> Reading<BlendShapeKeyformBindingOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<BlendShapeKeyformBindingOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
