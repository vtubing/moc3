use super::v4_02_00::offsets as v4_02_00;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct RotationDeformerKeyformOffsets {
  pub opacities: Offset,
  pub angles: Offset,
  pub origin_x: Offset,
  pub origin_y: Offset,
  pub scales: Offset,
  pub is_reflect_x: Offset,
  pub is_reflect_y: Offset,
  pub keyform_multiply_color_sources_begin_indices: Offset,
  pub keyform_screen_color_sources_begin_indices: Offset,
}

impl From<[Offset; 9]> for RotationDeformerKeyformOffsets {
  fn from([opacities, angles, origin_x, origin_y, scales, is_reflect_x, is_reflect_y, keyform_multiply_color_sources_begin_indices, keyform_screen_color_sources_begin_indices]: [Offset; 9]) -> Self {
    Self {
      opacities,
      angles,
      origin_x,
      origin_y,
      scales,
      is_reflect_x,
      is_reflect_y,
      keyform_multiply_color_sources_begin_indices,
      keyform_screen_color_sources_begin_indices,
    }
  }
}

impl From<(v4_02_00::RotationDeformerKeyformOffsets, [Offset; 2])> for RotationDeformerKeyformOffsets {
  fn from(
    (
      v4_02_00::RotationDeformerKeyformOffsets {
        opacities,
        angles,
        origin_x,
        origin_y,
        scales,
        is_reflect_x,
        is_reflect_y,
      },
      [keyform_multiply_color_sources_begin_indices, keyform_screen_color_sources_begin_indices],
    ): (v4_02_00::RotationDeformerKeyformOffsets, [Offset; 2]),
  ) -> Self {
    [
      opacities,
      angles,
      origin_x,
      origin_y,
      scales,
      is_reflect_x,
      is_reflect_y,
      keyform_multiply_color_sources_begin_indices,
      keyform_screen_color_sources_begin_indices,
    ]
    .into()
  }
}
