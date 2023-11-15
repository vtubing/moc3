use super::offsets::GlueKeyformOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct GlueKeyform {
  pub intensity: f32,
}

impl ExtractFromOffsets for GlueKeyform {
  type Offsets = GlueKeyformOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      intensity: moc3.read_one_at_offset_with_index(offsets.intensities, index)?,
    })
  }
}
