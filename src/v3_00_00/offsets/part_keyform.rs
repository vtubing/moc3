use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct PartKeyformOffsets {
  pub draw_orders: Offset,
}

impl From<[Offset; 1]> for PartKeyformOffsets {
  fn from([draw_orders]: [Offset; 1]) -> Self {
    Self { draw_orders }
  }
}

impl<T> Reading<PartKeyformOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<PartKeyformOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
