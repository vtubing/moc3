use super::offsets::DrawOrderGroupOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct DrawOrderGroup {
  pub object_sources_begin_index: i32,
  pub object_sources_count: i32,
  pub object_sources_total_count: i32,
  pub maximum_draw_order: u32,
  pub minimum_draw_order: u32,
}

impl ExtractFromOffsets for DrawOrderGroup {
  type Offsets = DrawOrderGroupOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      object_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.object_sources_begin_indices, index)?,
      object_sources_count: moc3.read_one_at_offset_with_index(offsets.object_sources_counts, index)?,
      object_sources_total_count: moc3.read_one_at_offset_with_index(offsets.object_sources_total_counts, index)?,
      maximum_draw_order: moc3.read_one_at_offset_with_index(offsets.maximum_draw_orders, index)?,
      minimum_draw_order: moc3.read_one_at_offset_with_index(offsets.minimum_draw_orders, index)?,
    })
  }
}
