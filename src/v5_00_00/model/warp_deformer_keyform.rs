use super::offsets::WarpDeformerKeyformOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WarpDeformerKeyform {
  pub opacity: f32,
  pub keyform_position_sources_begin_index: i32,
  pub keyform_multiply_color_sources_begin_index: i32,
  pub keyform_screen_color_sources_begin_index: i32,
}

impl ExtractFromOffsets for WarpDeformerKeyform {
  type Offsets = WarpDeformerKeyformOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      opacity: moc3.read_one_at_offset_with_index(offsets.opacities, index)?,
      keyform_position_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_position_sources_begin_indices, index)?,
      keyform_multiply_color_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_multiply_color_sources_begin_indices, index)?,
      keyform_screen_color_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keyform_screen_color_sources_begin_indices, index)?,
    })
  }
}
