use crate::prelude::{Reading, Result};

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Offset(pub u64);

impl std::ops::Deref for Offset {
  type Target = u64;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::fmt::Debug for Offset {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("@{:#010X}", self.0))
  }
}

impl<T> Reading<Offset> for T
where
  T: Reading<u32>,
{
  const SIZE: usize = <T as Reading<u32>>::SIZE;

  fn read_one(&mut self) -> Result<Offset> {
    self.read_one().map(u64::from).map(Offset)
  }
}
