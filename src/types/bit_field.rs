use crate::prelude::{Reading, Result};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BitField(pub [bool; 8]);

impl From<[bool; 8]> for BitField {
  fn from(bits: [bool; 8]) -> Self {
    Self(bits)
  }
}
impl From<u8> for BitField {
  fn from(bits: u8) -> Self {
    [
      bits & 0b00000001 != 0,
      bits & 0b00000010 != 0,
      bits & 0b00000100 != 0,
      bits & 0b00001000 != 0,
      bits & 0b00010000 != 0,
      bits & 0b00100000 != 0,
      bits & 0b01000000 != 0,
      bits & 0b10000000 != 0,
    ]
    .into()
  }
}

impl<T> Reading<BitField> for T
where
  T: Reading<u8>,
{
  const SIZE: usize = <T as Reading<u8>>::SIZE;

  fn read_one(&mut self) -> Result<BitField> {
    self.read_one().map(BitField::from)
  }
}
