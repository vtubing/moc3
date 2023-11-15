use super::offsets::PartKeyformOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct PartKeyform {
  pub draw_order: f32,
}

impl ExtractFromOffsets for PartKeyform {
  type Offsets = PartKeyformOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      draw_order: moc3.read_one_at_offset_with_index(offsets.draw_orders, index)?,
    })
  }
}
