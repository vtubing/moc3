use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct DrawableMaskOffsets {
  pub art_mesh_sources_indices: Offset,
}

impl From<[Offset; 1]> for DrawableMaskOffsets {
  fn from([art_mesh_sources_indices]: [Offset; 1]) -> Self {
    Self { art_mesh_sources_indices }
  }
}

impl<T> Reading<DrawableMaskOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<DrawableMaskOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
