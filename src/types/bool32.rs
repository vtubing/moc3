use crate::prelude::{Reading, Result};

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Bool32(pub bool);

impl std::ops::Deref for Bool32 {
  type Target = bool;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::fmt::Debug for Bool32 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Bool32({})", self.0))
  }
}

impl<T> Reading<Bool32> for T
where
  T: Reading<u32>,
{
  const SIZE: usize = <T as Reading<u32>>::SIZE;

  fn read_one(&mut self) -> Result<Bool32> {
    self.read_one().map(|value| value > 0).map(Bool32)
  }
}
