use crate::prelude::*;

use crate::v3_00_00;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct CountInfoTable {
  pub parts: u32,
  pub deformers: u32,
  pub warp_deformers: u32,
  pub rotation_deformers: u32,
  pub art_meshes: u32,
  pub parameters: u32,
  pub part_keyforms: u32,
  pub warp_deformer_keyforms: u32,
  pub rotation_deformer_keyforms: u32,
  pub art_mesh_keyforms: u32,
  pub keyform_positions: u32,
  pub parameter_binding_indices: u32,
  pub keyform_bindings: u32,
  pub parameter_bindings: u32,
  pub keys: u32,
  pub uvs: u32,
  pub position_indices: u32,
  pub drawable_masks: u32,
  pub draw_order_groups: u32,
  pub draw_order_group_objects: u32,
  pub glue: u32,
  pub glue_info: u32,
  pub glue_keyforms: u32,
  pub keyform_multiply_colors: u32,
  pub keyform_screen_colors: u32,
  pub blend_shape_parameter_bindings: u32,
  pub blend_shape_keyform_bindings: u32,
  pub blend_shapes_warp_deformers: u32,
  pub blend_shapes_art_meshes: u32,
  pub blend_shape_constraint_indices: u32,
  pub blend_shape_constraints: u32,
  pub blend_shape_constraint_values: u32,
}

impl CountInfoTable {
  pub fn read(moc3: &mut Reader) -> Result<Self> {
    let v3_00_00::CountInfoTable {
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
    } = v3_00_00::CountInfoTable::read(moc3)?;

    info!("{:#010x} CountInfoTable v4.02.00", moc3.stream_position()?);

    let keyform_multiply_colors = moc3.read_one()?;
    let keyform_screen_colors = moc3.read_one()?;
    let blend_shape_parameter_bindings = moc3.read_one()?;
    let blend_shape_keyform_bindings = moc3.read_one()?;
    let blend_shapes_warp_deformers = moc3.read_one()?;
    let blend_shapes_art_meshes = moc3.read_one()?;
    let blend_shape_constraint_indices = moc3.read_one()?;
    let blend_shape_constraints = moc3.read_one()?;
    let blend_shape_constraint_values = moc3.read_one()?;

    Ok(Self {
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
      keyform_multiply_colors,
      keyform_screen_colors,
      blend_shape_parameter_bindings,
      blend_shape_keyform_bindings,
      blend_shapes_warp_deformers,
      blend_shapes_art_meshes,
      blend_shape_constraint_indices,
      blend_shape_constraints,
      blend_shape_constraint_values,
    })
  }
}
