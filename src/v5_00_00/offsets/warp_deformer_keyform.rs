use super::v4_02_00::offsets as v4_02_00;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WarpDeformerKeyformOffsets {
  pub opacities: Offset,
  pub keyform_position_sources_begin_indices: Offset,
  pub keyform_multiply_color_sources_begin_indices: Offset,
  pub keyform_screen_color_sources_begin_indices: Offset,
}

impl From<[Offset; 4]> for WarpDeformerKeyformOffsets {
  fn from([opacities, keyform_position_sources_begin_indices, keyform_multiply_color_sources_begin_indices, keyform_screen_color_sources_begin_indices]: [Offset; 4]) -> Self {
    Self {
      opacities,
      keyform_position_sources_begin_indices,
      keyform_multiply_color_sources_begin_indices,
      keyform_screen_color_sources_begin_indices,
    }
  }
}

impl From<(v4_02_00::WarpDeformerKeyformOffsets, [Offset; 2])> for WarpDeformerKeyformOffsets {
  fn from(
    (
      v4_02_00::WarpDeformerKeyformOffsets {
        opacities,
        keyform_position_sources_begin_indices,
      },
      [keyform_multiply_color_sources_begin_indices, keyform_screen_color_sources_begin_indices],
    ): (v4_02_00::WarpDeformerKeyformOffsets, [Offset; 2]),
  ) -> Self {
    [
      opacities,
      keyform_position_sources_begin_indices,
      keyform_multiply_color_sources_begin_indices,
      keyform_screen_color_sources_begin_indices,
    ]
    .into()
  }
}
