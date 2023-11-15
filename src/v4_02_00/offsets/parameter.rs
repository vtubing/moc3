use super::v4_00_00::offsets as v4_00_00;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct ParameterOffsets {
  pub runtime_space_0: Offset,
  pub ids: Offset,
  pub max_values: Offset,
  pub min_values: Offset,
  pub default_values: Offset,
  pub is_repeat: Offset,
  pub decimal_places: Offset,
  pub parameter_binding_sources_begin_indices: Offset,
  pub parameter_binding_sources_counts: Offset,
  pub parameter_types: Offset,
  pub extension_runtime_space_0: Offset,
  pub keys_sources_begin_indices: Offset,
  pub keys_sources_counts: Offset,
  pub blend_shape_parameter_binding_sources_begin_indices: Offset,
  pub blend_shape_parameter_binding_sources_counts: Offset,
}

impl From<[Offset; 15]> for ParameterOffsets {
  fn from(
    [
      runtime_space_0,
      ids,
      max_values,
      min_values,
      default_values,
      is_repeat,
      decimal_places,
      parameter_binding_sources_begin_indices,
      parameter_binding_sources_counts,
      parameter_types,
      extension_runtime_space_0,
keys_sources_begin_indices,
keys_sources_counts,
      blend_shape_parameter_binding_sources_begin_indices,
      blend_shape_parameter_binding_sources_counts,
    ]: [Offset; 15],
  ) -> Self {
    Self {
      runtime_space_0,
      ids,
      max_values,
      min_values,
      default_values,
      is_repeat,
      decimal_places,
      parameter_binding_sources_begin_indices,
      parameter_binding_sources_counts,
      parameter_types,
      extension_runtime_space_0,
      keys_sources_begin_indices,
      keys_sources_counts,
      blend_shape_parameter_binding_sources_begin_indices,
      blend_shape_parameter_binding_sources_counts,
    }
  }
}

impl From<(v4_00_00::ParameterOffsets, super::ParameterExtensionOffsets, [Offset; 3])> for ParameterOffsets {
  fn from(
    (
      v4_00_00::ParameterOffsets {
        runtime_space_0,
        ids,
        max_values,
        min_values,
        default_values,
        is_repeat,
        decimal_places,
        parameter_binding_sources_begin_indices,
        parameter_binding_sources_counts,
      },
      super::ParameterExtensionOffsets {
        runtime_space_0: extension_runtime_space_0,
        keys_sources_begin_indices,
        keys_sources_counts,
      },
      [parameter_types, blend_shape_parameter_binding_sources_begin_indices, blend_shape_parameter_binding_sources_counts],
    ): (v4_00_00::ParameterOffsets, super::ParameterExtensionOffsets, [Offset; 3]),
  ) -> Self {
    [
      runtime_space_0,
      ids,
      max_values,
      min_values,
      default_values,
      is_repeat,
      decimal_places,
      parameter_binding_sources_begin_indices,
      parameter_binding_sources_counts,
      parameter_types,
      extension_runtime_space_0,
      keys_sources_begin_indices,
      keys_sources_counts,
      blend_shape_parameter_binding_sources_begin_indices,
      blend_shape_parameter_binding_sources_counts,
    ]
    .into()
  }
}
