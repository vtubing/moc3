use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ArtMeshKeyformOffsets {
  pub opacities: Offset,
  pub draw_orders: Offset,
  pub keyform_position_sources_begin_indices: Offset,
}

impl From<[Offset; 3]> for ArtMeshKeyformOffsets {
  fn from([opacities, draw_orders, keyform_position_sources_begin_indices]: [Offset; 3]) -> Self {
    Self {
      opacities,
      draw_orders,
      keyform_position_sources_begin_indices,
    }
  }
}

impl<T> Reading<ArtMeshKeyformOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<ArtMeshKeyformOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
