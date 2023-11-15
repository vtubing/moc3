use super::offsets::KeyformBindingOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct KeyformBinding {
  pub parameter_binding_index_sources_begin_index: i32,
  pub parameter_binding_index_sources_count: i32,
}

impl ExtractFromOffsets for KeyformBinding {
  type Offsets = KeyformBindingOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      parameter_binding_index_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.parameter_binding_index_sources_begin_indices, index)?,
      parameter_binding_index_sources_count: moc3.read_one_at_offset_with_index(offsets.parameter_binding_index_sources_counts, index)?,
    })
  }
}
