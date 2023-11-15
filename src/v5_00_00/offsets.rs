use crate::prelude::*;

use super::*;

pub use v4_02_00::offsets::{
  ArtMeshOffsets, BlendShapeConstraintIndicesOffsets, BlendShapeConstraintOffsets, BlendShapeConstraintValueOffsets, BlendShapeKeyformBindingOffsets, BlendShapeOffsets,
  BlendShapeParameterBindingOffsets, DeformerOffsets, DrawOrderGroupObjectOffsets, DrawOrderGroupOffsets, DrawableMaskOffsets, GlueInfoOffsets, GlueKeyformOffsets, GlueOffsets, KeyOffsets,
  KeyformBindingOffsets, KeyformColorOffsets, KeyformPositionOffsets, ParameterBindingIndicesOffsets, ParameterBindingOffsets, ParameterExtensionOffsets, ParameterOffsets, PartKeyformOffsets,
  PartOffsets, PositionIndicesOffsets, RotationDeformerOffsets, UvOffsets, WarpDeformerOffsets,
};

mod art_mesh_keyform;
mod rotation_deformer_keyform;
mod warp_deformer_keyform;

pub use art_mesh_keyform::ArtMeshKeyformOffsets;
pub use rotation_deformer_keyform::RotationDeformerKeyformOffsets;
pub use warp_deformer_keyform::WarpDeformerKeyformOffsets;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct SectionOffsetTable {
  pub count_info: Offset,
  pub canvas_info: Offset,
  pub parts: PartOffsets,
  pub deformers: DeformerOffsets,
  pub warp_deformers: WarpDeformerOffsets,
  pub rotation_deformers: RotationDeformerOffsets,
  pub art_meshes: ArtMeshOffsets,
  pub parameters: ParameterOffsets,
  pub part_keyforms: PartKeyformOffsets,
  pub warp_deformer_keyforms: WarpDeformerKeyformOffsets,
  pub rotation_deformer_keyforms: RotationDeformerKeyformOffsets,
  pub art_mesh_keyforms: ArtMeshKeyformOffsets,
  pub keyform_positions: KeyformPositionOffsets,
  pub parameter_binding_indices: ParameterBindingIndicesOffsets,
  pub keyform_bindings: KeyformBindingOffsets,
  pub parameter_bindings: ParameterBindingOffsets,
  pub keys: KeyOffsets,
  pub uvs: UvOffsets,
  pub position_indices: PositionIndicesOffsets,
  pub drawable_masks: DrawableMaskOffsets,
  pub draw_order_groups: DrawOrderGroupOffsets,
  pub draw_order_group_objects: DrawOrderGroupObjectOffsets,
  pub glue: GlueOffsets,
  pub glue_info: GlueInfoOffsets,
  pub glue_keyforms: GlueKeyformOffsets,
  pub parameter_extensions: ParameterExtensionOffsets,
  pub keyform_multiply_colors: KeyformColorOffsets,
  pub keyform_screen_colors: KeyformColorOffsets,
  pub blend_shape_parameter_bindings: BlendShapeParameterBindingOffsets,
  pub blend_shape_keyform_binding_offsets: BlendShapeKeyformBindingOffsets,
  pub blend_shapes_warp_deformers: BlendShapeOffsets,
  pub blend_shapes_art_meshes: BlendShapeOffsets,
  pub blend_shape_constraint_indices_offsets: BlendShapeConstraintIndicesOffsets,
  pub blend_shape_constraint_offsets: BlendShapeConstraintOffsets,
  pub blend_shape_constraint_value_offsets: BlendShapeConstraintValueOffsets,
  pub blend_shapes_parts: BlendShapeOffsets,
  pub blend_shapes_rotation_deformers: BlendShapeOffsets,
  pub blend_shapes_glue: BlendShapeOffsets,
}

impl SectionOffsetTable {
  pub fn read(moc3: &mut Reader) -> Result<Self> {
    let v4_02_00::SectionOffsetTable {
      art_mesh_keyforms,
      art_meshes,
      blend_shapes_art_meshes,
      blend_shape_constraint_indices_offsets,
      blend_shape_constraint_offsets,
      blend_shape_constraint_value_offsets,
      blend_shape_keyform_binding_offsets,
      blend_shape_parameter_bindings,
      blend_shapes_warp_deformers,
      canvas_info,
      count_info,
      deformers,
      draw_order_group_objects,
      draw_order_groups,
      drawable_masks,
      glue,
      glue_info,
      glue_keyforms,
      keyform_bindings,
      keyform_multiply_colors,
      keyform_positions,
      keyform_screen_colors,
      keys,
      parameter_binding_indices,
      parameter_bindings,
      parameter_extensions,
      parameters,
      part_keyforms,
      parts,
      position_indices,
      rotation_deformer_keyforms,
      rotation_deformers,
      uvs,
      warp_deformer_keyforms,
      warp_deformers,
    } = v4_02_00::SectionOffsetTable::read(moc3)?;

    info!("{:#010x} SectionOffsetTable v5.00.00", moc3.stream_position()?);

    let warp_deformer_keyforms = (warp_deformer_keyforms, moc3.read()?).into();
    let rotation_deformer_keyforms = (rotation_deformer_keyforms, moc3.read()?).into();
    let art_mesh_keyforms = (art_mesh_keyforms, moc3.read()?).into();

    let blend_shapes_parts = moc3.read_one()?;
    let blend_shapes_rotation_deformers = moc3.read_one()?;
    let blend_shapes_glue = moc3.read_one()?;

    Ok(Self {
      count_info,
      canvas_info,
      parts,
      deformers,
      warp_deformers,
      rotation_deformers,
      art_meshes,
      parameters,
      part_keyforms,
      warp_deformer_keyforms,
      rotation_deformer_keyforms,
      art_mesh_keyforms,
      keyform_positions,
      parameter_binding_indices,
      keyform_bindings,
      parameter_bindings,
      keys,
      uvs,
      position_indices,
      drawable_masks,
      draw_order_groups,
      draw_order_group_objects,
      glue,
      glue_info,
      glue_keyforms,
      parameter_extensions,
      keyform_multiply_colors,
      keyform_screen_colors,
      blend_shape_parameter_bindings,
      blend_shape_keyform_binding_offsets,
      blend_shapes_warp_deformers,
      blend_shapes_art_meshes,
      blend_shape_constraint_indices_offsets,
      blend_shape_constraint_offsets,
      blend_shape_constraint_value_offsets,
      blend_shapes_parts,
      blend_shapes_rotation_deformers,
      blend_shapes_glue,
    })
  }
}
