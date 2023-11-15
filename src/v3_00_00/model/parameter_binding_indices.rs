use super::offsets::ParameterBindingIndicesOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ParameterBindingIndices {
  pub binding_sources_indices: i32,
}

impl ExtractFromOffsets for ParameterBindingIndices {
  type Offsets = ParameterBindingIndicesOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      binding_sources_indices: moc3.read_one_at_offset_with_index(offsets.binding_sources_indices, index)?,
    })
  }
}
