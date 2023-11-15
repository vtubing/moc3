use super::v4_00_00::offsets as v4_00_00;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct RotationDeformerOffsets {
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub base_angles: Offset,
  pub keyform_color_sources_begin_indices: Offset,
}

impl From<[Offset; 5]> for RotationDeformerOffsets {
  fn from([keyform_binding_sources_indices, keyform_sources_begin_indices, keyform_sources_counts, base_angles, keyform_color_sources_begin_indices]: [Offset; 5]) -> Self {
    Self {
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      base_angles,
      keyform_color_sources_begin_indices,
    }
  }
}

impl From<(v4_00_00::RotationDeformerOffsets, [Offset; 1])> for RotationDeformerOffsets {
  fn from(
    (
      v4_00_00::RotationDeformerOffsets {
        keyform_binding_sources_indices,
        keyform_sources_begin_indices,
        keyform_sources_counts,
        base_angles,
      },
      [keyform_color_sources_begin_indices],
    ): (v4_00_00::RotationDeformerOffsets, [Offset; 1]),
  ) -> Self {
    [
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      base_angles,
      keyform_color_sources_begin_indices,
    ]
    .into()
  }
}
