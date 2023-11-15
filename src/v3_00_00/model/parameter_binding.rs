use super::offsets::ParameterBindingOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ParameterBinding {
  pub keys_sources_begin_index: i32,
  pub keys_sources_count: i32,
}

impl ExtractFromOffsets for ParameterBinding {
  type Offsets = ParameterBindingOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      keys_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keys_sources_begin_indices, index)?,
      keys_sources_count: moc3.read_one_at_offset_with_index(offsets.keys_sources_counts, index)?,
    })
  }
}
