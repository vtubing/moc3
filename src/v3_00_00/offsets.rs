use crate::prelude::*;

mod art_mesh;
mod art_mesh_keyform;
mod deformer;
mod draw_order_group;
mod draw_order_group_object;
mod drawable_mask;
mod glue;
mod glue_info;
mod glue_keyform;
mod key;
mod keyform_binding;
mod keyform_position;
mod parameter;
mod parameter_binding;
mod parameter_binding_indices;
mod part;
mod part_keyform;
mod position_indices;
mod rotation_deformer;
mod rotation_deformer_keyform;
mod uv;
mod warp_deformer;
mod warp_deformer_keyform;

pub use art_mesh::ArtMeshOffsets;
pub use art_mesh_keyform::ArtMeshKeyformOffsets;
pub use deformer::DeformerOffsets;
pub use draw_order_group::DrawOrderGroupOffsets;
pub use draw_order_group_object::DrawOrderGroupObjectOffsets;
pub use drawable_mask::DrawableMaskOffsets;
pub use glue::GlueOffsets;
pub use glue_info::GlueInfoOffsets;
pub use glue_keyform::GlueKeyformOffsets;
pub use key::KeyOffsets;
pub use keyform_binding::KeyformBindingOffsets;
pub use keyform_position::KeyformPositionOffsets;
pub use parameter::ParameterOffsets;
pub use parameter_binding::ParameterBindingOffsets;
pub use parameter_binding_indices::ParameterBindingIndicesOffsets;
pub use part::PartOffsets;
pub use part_keyform::PartKeyformOffsets;
pub use position_indices::PositionIndicesOffsets;
pub use rotation_deformer::RotationDeformerOffsets;
pub use rotation_deformer_keyform::RotationDeformerKeyformOffsets;
pub use uv::UvOffsets;
pub use warp_deformer::WarpDeformerOffsets;
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
}

impl SectionOffsetTable {
  pub fn read(moc3: &mut Reader) -> Result<Self> {
    info!("{:#010x} SectionOffsetTable v3.00.00", moc3.stream_position()?);

    let count_info = moc3.read_one()?;
    let canvas_info = moc3.read_one()?;
    let parts = moc3.read_one()?;
    let deformers = moc3.read_one()?;
    let warp_deformers = moc3.read_one()?;
    let rotation_deformers = moc3.read_one()?;
    let art_meshes = moc3.read_one()?;
    let parameters = moc3.read_one()?;
    let part_keyforms = moc3.read_one()?;
    let warp_deformer_keyforms = moc3.read_one()?;
    let rotation_deformer_keyforms = moc3.read_one()?;
    let art_mesh_keyforms = moc3.read_one()?;
    let keyform_positions = moc3.read_one()?;
    let parameter_binding_indices = moc3.read_one()?;
    let keyform_bindings = moc3.read_one()?;
    let parameter_bindings = moc3.read_one()?;
    let keys = moc3.read_one()?;
    let uvs = moc3.read_one()?;
    let position_indices = moc3.read_one()?;
    let drawable_masks = moc3.read_one()?;
    let draw_order_groups = moc3.read_one()?;
    let draw_order_group_objects = moc3.read_one()?;
    let glue = moc3.read_one()?;
    let glue_info = moc3.read_one()?;
    let glue_keyform = moc3.read_one()?;

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
      glue_keyforms: glue_keyform,
    })
  }
}
