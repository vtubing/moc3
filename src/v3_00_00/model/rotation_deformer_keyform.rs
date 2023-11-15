use super::offsets::RotationDeformerKeyformOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct RotationDeformerKeyform {
  pub opacity: f32,
  pub angle: f32,
  pub origin_x: f32,
  pub origin_y: f32,
  pub scale: f32,
  pub is_reflect_x: Bool32,
  pub is_reflect_y: Bool32,
}

impl ExtractFromOffsets for RotationDeformerKeyform {
  type Offsets = RotationDeformerKeyformOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      opacity: moc3.read_one_at_offset_with_index(offsets.opacities, index)?,
      angle: moc3.read_one_at_offset_with_index(offsets.angles, index)?,
      origin_x: moc3.read_one_at_offset_with_index(offsets.origin_x, index)?,
      origin_y: moc3.read_one_at_offset_with_index(offsets.origin_y, index)?,
      scale: moc3.read_one_at_offset_with_index(offsets.scales, index)?,
      is_reflect_x: moc3.read_one_at_offset_with_index(offsets.is_reflect_x, index)?,
      is_reflect_y: moc3.read_one_at_offset_with_index(offsets.is_reflect_y, index)?,
    })
  }
}
