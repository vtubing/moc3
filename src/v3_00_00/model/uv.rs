use super::offsets::UvOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Uv {
  pub u: f32,
  pub v: f32,
}

impl ExtractFromOffsets for Uv {
  type Offsets = UvOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    moc3.read_one_at_offset_with_index(offsets.uvs, index)
  }
}

impl<T> Reading<Uv> for T
where
  T: Reading<f32>,
{
  fn read_one(&mut self) -> Result<Uv> {
    let [u, v] = self.read()?;
    Ok(Uv { u, v })
  }
}
