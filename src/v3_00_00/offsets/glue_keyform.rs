use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct GlueKeyformOffsets {
  pub intensities: Offset,
}

impl From<[Offset; 1]> for GlueKeyformOffsets {
  fn from([intensities]: [Offset; 1]) -> Self {
    Self { intensities }
  }
}

impl<T> Reading<GlueKeyformOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<GlueKeyformOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
