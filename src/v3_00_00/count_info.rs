use crate::prelude::*;

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
}

impl From<[u32; 23]> for CountInfoTable {
  fn from(
    [parts,
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
    glue_keyforms,]: [u32; 23],
  ) -> Self {
    Self {
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
    }
  }
}

impl CountInfoTable {
  pub fn read(moc3: &mut Reader) -> Result<Self> {
    Ok(moc3.read()?.into())
  }
}
