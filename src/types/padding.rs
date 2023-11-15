use crate::prelude::{Reading, Result};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Padding<const N: usize>(pub [u8; N]);

impl<const N: usize> From<[u8; N]> for Padding<N> {
  fn from(bytes: [u8; N]) -> Self {
    Self(bytes)
  }
}

impl<const N: usize> Default for Padding<N> {
  fn default() -> Self {
    Self([0u8; N])
  }
}

impl<const N: usize> std::fmt::Debug for Padding<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_empty() {
      f.write_fmt(format_args!("Padding({} bytes) - EMPTY", N))
    } else {
      f.write_fmt(format_args!("Padding({} bytes) - NOT EMPTY", N))
    }
  }
}

impl<const N: usize> Padding<N> {
  pub fn is_empty(&self) -> bool {
    self.0.iter().all(|value| value == &0)
  }
}

impl<T, const N: usize> Reading<Padding<N>> for T
where
  T: Reading<u8>,
{
  const SIZE: usize = <T as Reading<u8>>::SIZE * N;

  fn read_one(&mut self) -> Result<Padding<N>> {
    let bytes = self.read()?;
    Ok(Padding(bytes))
  }
}
