use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct DrawOrderGroupOffsets {
  pub object_sources_begin_indices: Offset,
  pub object_sources_counts: Offset,
  pub object_sources_total_counts: Offset,
  pub maximum_draw_orders: Offset,
  pub minimum_draw_orders: Offset,
}

impl From<[Offset; 5]> for DrawOrderGroupOffsets {
  fn from([object_sources_begin_indices, object_sources_counts, object_sources_total_counts, maximum_draw_orders, minimum_draw_orders]: [Offset; 5]) -> Self {
    Self {
      object_sources_begin_indices,
      object_sources_counts,
      object_sources_total_counts,
      maximum_draw_orders,
      minimum_draw_orders,
    }
  }
}

impl<T> Reading<DrawOrderGroupOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<DrawOrderGroupOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
