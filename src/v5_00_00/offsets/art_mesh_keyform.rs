use super::v4_02_00::offsets as v4_02_00;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ArtMeshKeyformOffsets {
  pub opacities: Offset,
  pub draw_orders: Offset,
  pub keyform_position_sources_begin_indices: Offset,
  pub keyform_multiply_color_sources_begin_indices: Offset,
  pub keyform_screen_color_sources_begin_indices: Offset,
}

impl From<[Offset; 5]> for ArtMeshKeyformOffsets {
  fn from([opacities, draw_orders, keyform_position_sources_begin_indices, keyform_multiply_color_sources_begin_indices, keyform_screen_color_sources_begin_indices]: [Offset; 5]) -> Self {
    Self {
      opacities,
      draw_orders,
      keyform_position_sources_begin_indices,
      keyform_multiply_color_sources_begin_indices,
      keyform_screen_color_sources_begin_indices,
    }
  }
}

impl From<(v4_02_00::ArtMeshKeyformOffsets, [Offset; 2])> for ArtMeshKeyformOffsets {
  fn from(
    (
      v4_02_00::ArtMeshKeyformOffsets {
        opacities,
        draw_orders,
        keyform_position_sources_begin_indices,
      },
      [keyform_multiply_color_sources_begin_indices, keyform_screen_color_sources_begin_indices],
    ): (v4_02_00::ArtMeshKeyformOffsets, [Offset; 2]),
  ) -> Self {
    [
      opacities,
      draw_orders,
      keyform_position_sources_begin_indices,
      keyform_multiply_color_sources_begin_indices,
      keyform_screen_color_sources_begin_indices,
    ]
    .into()
  }
}
