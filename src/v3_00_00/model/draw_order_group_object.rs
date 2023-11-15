use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct DrawOrderGroupObject {
  pub type_: DrawOrderGroupObjectType,
  pub index: i32,
  pub self_index: i32,
}

impl ExtractFromOffsets for DrawOrderGroupObject {
  type Offsets = super::offsets::DrawOrderGroupObjectOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      type_: moc3.read_one_at_offset_with_index(offsets.types, index)?,
      index: moc3.read_one_at_offset_with_index(offsets.indices, index)?,
      self_index: moc3.read_one_at_offset_with_index(offsets.self_indices, index)?,
    })
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum DrawOrderGroupObjectType {
  #[default]
  ArtMesh = 0,
  Part = 1,
}

impl TryFrom<u32> for DrawOrderGroupObjectType {
  type Error = Error;

  fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::ArtMesh),
      1 => Ok(Self::Part),
      _ => Err(Error::UnknownDrawOrderGroupObjectType(value)),
    }
  }
}

impl<T> Reading<DrawOrderGroupObjectType> for T
where
  T: Reading<u32>,
{
  const SIZE: usize = <T as Reading<u32>>::SIZE;
  fn read_one(&mut self) -> Result<DrawOrderGroupObjectType> {
    self.read_one()?.try_into()
  }
}
