use super::offsets::ParameterOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Parameter {
  pub id: ID,
  pub max_value: f32,
  pub min_value: f32,
  pub default_value: f32,
  pub is_repeat: Bool32,
  pub decimal_places: u32,
  pub parameter_binding_sources_begin_index: i32,
  pub parameter_binding_sources_count: i32,
  pub keys_sources_begin_index: i32,
  pub keys_sources_count: i32,
  pub blend_shape_parameter_binding_sources_begin_index: i32,
  pub blend_shape_parameter_binding_sources_count: i32,
}

impl ExtractFromOffsets for Parameter {
  type Offsets = ParameterOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      id: moc3.read_one_at_offset_with_index(offsets.ids, index)?,
      max_value: moc3.read_one_at_offset_with_index(offsets.max_values, index)?,
      min_value: moc3.read_one_at_offset_with_index(offsets.min_values, index)?,
      default_value: moc3.read_one_at_offset_with_index(offsets.default_values, index)?,
      is_repeat: moc3.read_one_at_offset_with_index(offsets.is_repeat, index)?,
      decimal_places: moc3.read_one_at_offset_with_index(offsets.decimal_places, index)?,
      parameter_binding_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.parameter_binding_sources_begin_indices, index)?,
      parameter_binding_sources_count: moc3.read_one_at_offset_with_index(offsets.parameter_binding_sources_counts, index)?,
      keys_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.keys_sources_begin_indices, index)?,
      keys_sources_count: moc3.read_one_at_offset_with_index(offsets.keys_sources_counts, index)?,
      blend_shape_parameter_binding_sources_begin_index: moc3.read_one_at_offset_with_index(offsets.blend_shape_parameter_binding_sources_begin_indices, index)?,
      blend_shape_parameter_binding_sources_count: moc3.read_one_at_offset_with_index(offsets.blend_shape_parameter_binding_sources_counts, index)?,
    })
  }
}
