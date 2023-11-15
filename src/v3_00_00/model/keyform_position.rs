use super::offsets::KeyformPositionOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct KeyformPosition {
  pub x: f32,
  pub y: f32,
}

impl ExtractFromOffsets for KeyformPosition {
  type Offsets = KeyformPositionOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    moc3.read_one_at_offset_with_index(offsets.xys, index)
  }
}

impl<T> Reading<KeyformPosition> for T
where
  T: Reading<f32>,
{
  fn read_one(&mut self) -> Result<KeyformPosition> {
    let [x, y] = self.read()?;
    Ok(KeyformPosition { x, y })
  }
}
