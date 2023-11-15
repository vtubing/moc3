use super::offsets::DeformerOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Deformer {
  pub id: ID,
  pub keyform_binding_source_index: i32,
  pub is_visible: Bool32,
  pub is_enabled: Bool32,
  pub parent_part_index: i32,
  pub parent_deformer_index: i32,
  pub type_: DeformerType,
  pub specific_sources_index: i32,
}

impl ExtractFromOffsets for Deformer {
  type Offsets = DeformerOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      id: moc3.read_one_at_offset_with_index(offsets.ids, index)?,
      keyform_binding_source_index: moc3.read_one_at_offset_with_index(offsets.keyform_binding_sources_indices, index)?,
      is_visible: moc3.read_one_at_offset_with_index(offsets.is_visible, index)?,
      is_enabled: moc3.read_one_at_offset_with_index(offsets.is_enabled, index)?,
      parent_part_index: moc3.read_one_at_offset_with_index(offsets.parent_part_indices, index)?,
      parent_deformer_index: moc3.read_one_at_offset_with_index(offsets.parent_deformer_indices, index)?,
      type_: moc3.read_one_at_offset_with_index(offsets.types, index)?,
      specific_sources_index: moc3.read_one_at_offset_with_index(offsets.specific_sources_indices, index)?,
    })
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum DeformerType {
  #[default]
  Warp = 0,
  Rotation = 1,
}

impl TryFrom<u32> for DeformerType {
  type Error = Error;
  fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
    match value {
      0 => Ok(DeformerType::Warp),
      1 => Ok(DeformerType::Rotation),
      _ => Err(Error::UnknownDeformerType(value)),
    }
  }
}

impl<T> Reading<DeformerType> for T
where
  T: Reading<u32>,
{
  const SIZE: usize = <T as Reading<u32>>::SIZE;

  fn read_one(&mut self) -> Result<DeformerType> {
    self.read_one()?.try_into()
  }
}
