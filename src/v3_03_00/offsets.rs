use crate::prelude::*;

use super::v3_00_00;
pub use v3_00_00::offsets::{
  ArtMeshKeyformOffsets, ArtMeshOffsets, DeformerOffsets, DrawOrderGroupObjectOffsets, DrawOrderGroupOffsets, DrawableMaskOffsets, GlueInfoOffsets, GlueKeyformOffsets, GlueOffsets, KeyOffsets,
  KeyformBindingOffsets, KeyformPositionOffsets, ParameterBindingIndicesOffsets, ParameterBindingOffsets, ParameterOffsets, PartKeyformOffsets, PartOffsets, PositionIndicesOffsets,
  RotationDeformerKeyformOffsets, RotationDeformerOffsets, UvOffsets, WarpDeformerKeyformOffsets,
};

mod warp_deformer;

pub use warp_deformer::WarpDeformerOffsets;

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
}

impl SectionOffsetTable {
  pub fn read(moc3: &mut Reader) -> Result<Self> {
    let v3_00_00::SectionOffsetTable {
      art_mesh_keyforms,
      art_meshes,
      canvas_info,
      count_info,
      deformers,
      draw_order_group_objects,
      draw_order_groups,
      drawable_masks,
      glue_info,
      glue_keyforms,
      glue,
      keys,
      keyform_bindings,
      keyform_positions,
      parameter_binding_indices,
      parameter_bindings,
      parameters,
      part_keyforms,
      parts,
      position_indices,
      rotation_deformer_keyforms,
      rotation_deformers,
      uvs,
      warp_deformer_keyforms,
      warp_deformers,
    } = v3_00_00::SectionOffsetTable::read(moc3)?;

    info!("{:#010x} SectionOffsetTable v3.03.00", moc3.stream_position()?);

    let warp_deformers = (warp_deformers, moc3.read()?).into();

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
    })
  }
}
