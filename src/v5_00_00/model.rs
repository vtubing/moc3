use crate::prelude::*;

use super::*;
use crate::CanvasInfo;

pub use v4_02_00::model::{
  ArtMesh, ArtMeshDrawableFlags, BlendShape, BlendShapeConstraint, BlendShapeConstraintIndex, BlendShapeConstraintValue, BlendShapeKeyformBinding, BlendShapeParameterBinding, Deformer, DeformerType,
  DrawOrderGroup, DrawOrderGroupObject, DrawOrderGroupObjectType, DrawableMask, Glue, GlueInfo, GlueKeyform, Key, KeyformBinding, KeyformColor, KeyformPosition, Parameter, ParameterBinding,
  ParameterBindingIndices, Part, PartKeyform, PositionIndices, RotationDeformer, Uv, WarpDeformer,
};

mod art_mesh_keyform;
mod rotation_deformer_keyform;
mod warp_deformer_keyform;

use art_mesh_keyform::ArtMeshKeyform;
use rotation_deformer_keyform::RotationDeformerKeyform;
use warp_deformer_keyform::WarpDeformerKeyform;

#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub struct Model {
  pub header: Header,
  pub count: CountInfoTable,
  pub canvas: CanvasInfo,
  pub offsets: SectionOffsetTable,
  #[derivative(Debug = "ignore")]
  pub parts: Vec<Part>,
  #[derivative(Debug = "ignore")]
  pub deformers: Vec<Deformer>,
  #[derivative(Debug = "ignore")]
  pub warp_deformers: Vec<WarpDeformer>,
  #[derivative(Debug = "ignore")]
  pub rotation_deformers: Vec<RotationDeformer>,
  #[derivative(Debug = "ignore")]
  pub art_meshes: Vec<ArtMesh>,
  #[derivative(Debug = "ignore")]
  pub parameters: Vec<Parameter>,
  #[derivative(Debug = "ignore")]
  pub part_keyforms: Vec<PartKeyform>,
  #[derivative(Debug = "ignore")]
  pub warp_deformer_keyforms: Vec<WarpDeformerKeyform>,
  #[derivative(Debug = "ignore")]
  pub rotation_deformer_keyforms: Vec<RotationDeformerKeyform>,
  #[derivative(Debug = "ignore")]
  pub art_mesh_keyforms: Vec<ArtMeshKeyform>,
  #[derivative(Debug = "ignore")]
  pub keyform_positions: Vec<KeyformPosition>,
  #[derivative(Debug = "ignore")]
  pub parameter_binding_indices: Vec<ParameterBindingIndices>,
  #[derivative(Debug = "ignore")]
  pub keyform_bindings: Vec<KeyformBinding>,
  #[derivative(Debug = "ignore")]
  pub parameter_bindings: Vec<ParameterBinding>,
  #[derivative(Debug = "ignore")]
  pub keys: Vec<Key>,
  #[derivative(Debug = "ignore")]
  pub uvs: Vec<Uv>,
  #[derivative(Debug = "ignore")]
  pub position_indices: Vec<PositionIndices>,
  #[derivative(Debug = "ignore")]
  pub drawable_masks: Vec<DrawableMask>,
  #[derivative(Debug = "ignore")]
  pub draw_order_groups: Vec<DrawOrderGroup>,
  #[derivative(Debug = "ignore")]
  pub draw_order_group_objects: Vec<DrawOrderGroupObject>,
  #[derivative(Debug = "ignore")]
  pub glue: Vec<Glue>,
  #[derivative(Debug = "ignore")]
  pub glue_info: Vec<GlueInfo>,
  #[derivative(Debug = "ignore")]
  pub glue_keyforms: Vec<GlueKeyform>,
  #[derivative(Debug = "ignore")]
  pub keyform_multiply_colors: Vec<KeyformColor>,
  #[derivative(Debug = "ignore")]
  pub keyform_screen_colors: Vec<KeyformColor>,
  #[derivative(Debug = "ignore")]
  pub blend_shape_parameter_bindings: Vec<BlendShapeParameterBinding>,
  #[derivative(Debug = "ignore")]
  pub blend_shape_keyform_bindings: Vec<BlendShapeKeyformBinding>,
  #[derivative(Debug = "ignore")]
  pub blend_shapes_warp_deformers: Vec<BlendShape>,
  #[derivative(Debug = "ignore")]
  pub blend_shapes_art_deformers: Vec<BlendShape>,
  #[derivative(Debug = "ignore")]
  pub blend_shape_constraint_indices: Vec<BlendShapeConstraintIndex>,
  #[derivative(Debug = "ignore")]
  pub blend_shape_constraints: Vec<BlendShapeConstraint>,
  #[derivative(Debug = "ignore")]
  pub blend_shape_constraint_values: Vec<BlendShapeConstraintValue>,
  #[derivative(Debug = "ignore")]
  pub blend_shapes_parts: Vec<BlendShape>,
  #[derivative(Debug = "ignore")]
  pub blend_shapes_rotation_deformers: Vec<BlendShape>,
  #[derivative(Debug = "ignore")]
  pub blend_shapes_glue: Vec<BlendShape>,
}

impl Model {
  pub fn read(moc3: &mut Reader) -> Result<Self> {
    let header: Header = {
      moc3.seek(SeekFrom::Start(0x0))?;
      let mut sandbox = moc3.read_u8s_to_new::<64>()?;
      sandbox.read_one()?
    };

    let offsets = {
      moc3.seek(SeekFrom::Start(0x40))?;
      let mut sandbox = moc3.read_u8s_to_new::<0x280>()?;
      SectionOffsetTable::read(&mut sandbox)?
    };

    let count = {
      moc3.seek(SeekFrom::Start(*offsets.count_info))?;
      let mut sandbox = moc3.read_u8s_to_new::<256>()?;
      CountInfoTable::read(&mut sandbox)?
    };

    let canvas = {
      moc3.seek(SeekFrom::Start(*offsets.canvas_info))?;
      let mut sandbox = moc3.read_u8s_to_new::<64>()?;
      CanvasInfo::read(&mut sandbox)?
    };

    let parts = Part::extract(count.parts, &offsets.parts, moc3)?;
    let deformers = Deformer::extract(count.deformers, &offsets.deformers, moc3)?;
    let warp_deformers = WarpDeformer::extract(count.warp_deformers, &offsets.warp_deformers, moc3)?;
    let rotation_deformers = RotationDeformer::extract(count.rotation_deformers, &offsets.rotation_deformers, moc3)?;
    let art_meshes = ArtMesh::extract(count.art_meshes, &offsets.art_meshes, moc3)?;
    let parameters = Parameter::extract(count.parameters, &offsets.parameters, moc3)?;
    let part_keyforms = PartKeyform::extract(count.part_keyforms, &offsets.part_keyforms, moc3)?;
    let warp_deformer_keyforms = WarpDeformerKeyform::extract(count.warp_deformer_keyforms, &offsets.warp_deformer_keyforms, moc3)?;
    let rotation_deformer_keyforms = RotationDeformerKeyform::extract(count.rotation_deformer_keyforms, &offsets.rotation_deformer_keyforms, moc3)?;
    let art_mesh_keyforms = ArtMeshKeyform::extract(count.art_mesh_keyforms, &offsets.art_mesh_keyforms, moc3)?;
    let keyform_positions = KeyformPosition::extract(count.keyform_positions / 2, &offsets.keyform_positions, moc3)?;
    let parameter_binding_indices = ParameterBindingIndices::extract(count.parameter_binding_indices, &offsets.parameter_binding_indices, moc3)?;
    let keyform_bindings = KeyformBinding::extract(count.keyform_bindings, &offsets.keyform_bindings, moc3)?;
    let parameter_bindings = ParameterBinding::extract(count.parameter_bindings, &offsets.parameter_bindings, moc3)?;
    let keys = Key::extract(count.keys, &offsets.keys, moc3)?;
    let uvs = Uv::extract(count.uvs / 2, &offsets.uvs, moc3)?;
    let position_indices = PositionIndices::extract(count.position_indices, &offsets.position_indices, moc3)?;
    let drawable_masks = DrawableMask::extract(count.drawable_masks, &offsets.drawable_masks, moc3)?;
    let draw_order_groups = DrawOrderGroup::extract(count.draw_order_groups, &offsets.draw_order_groups, moc3)?;
    let draw_order_group_objects = DrawOrderGroupObject::extract(count.draw_order_group_objects, &offsets.draw_order_group_objects, moc3)?;
    let glue = Glue::extract(count.glue, &offsets.glue, moc3)?;
    let glue_info = GlueInfo::extract(count.glue_info, &offsets.glue_info, moc3)?;
    let glue_keyforms = GlueKeyform::extract(count.glue_keyforms, &offsets.glue_keyforms, moc3)?;
    let keyform_multiply_colors = KeyformColor::extract(count.keyform_multiply_colors, &offsets.keyform_multiply_colors, moc3)?;
    let keyform_screen_colors = KeyformColor::extract(count.keyform_screen_colors, &offsets.keyform_screen_colors, moc3)?;
    let blend_shape_parameter_bindings = BlendShapeParameterBinding::extract(count.blend_shape_parameter_bindings, &offsets.blend_shape_parameter_bindings, moc3)?;
    let blend_shape_keyform_bindings = BlendShapeKeyformBinding::extract(count.blend_shape_keyform_bindings, &offsets.blend_shape_keyform_binding_offsets, moc3)?;
    let blend_shapes_warp_deformers = BlendShape::extract(count.blend_shapes_warp_deformers, &offsets.blend_shapes_warp_deformers, moc3)?;
    let blend_shapes_art_deformers = BlendShape::extract(count.blend_shapes_art_meshes, &offsets.blend_shapes_art_meshes, moc3)?;
    let blend_shape_constraint_indices = BlendShapeConstraintIndex::extract(count.blend_shape_constraint_indices, &offsets.blend_shape_constraint_indices_offsets, moc3)?;
    let blend_shape_constraints = BlendShapeConstraint::extract(count.blend_shape_constraints, &offsets.blend_shape_constraint_offsets, moc3)?;
    let blend_shape_constraint_values = BlendShapeConstraintValue::extract(count.blend_shape_constraint_values, &offsets.blend_shape_constraint_value_offsets, moc3)?;
    let blend_shapes_parts = BlendShape::extract(count.blend_shapes_parts, &offsets.blend_shapes_parts, moc3)?;
    let blend_shapes_rotation_deformers = BlendShape::extract(count.blend_shapes_rotation_deformers, &offsets.blend_shapes_rotation_deformers, moc3)?;
    let blend_shapes_glue = BlendShape::extract(count.blend_shapes_glue, &offsets.blend_shapes_glue, moc3)?;

    Ok(Self {
      header,
      count,
      canvas,
      offsets,
      parts,
      deformers,
      warp_deformers,
      rotation_deformers,
      art_meshes,
      parameters,
      art_mesh_keyforms,
      draw_order_group_objects,
      draw_order_groups,
      drawable_masks,
      glue,
      glue_info,
      glue_keyforms,
      keyform_bindings,
      keyform_positions,
      keys,
      parameter_binding_indices,
      parameter_bindings,
      part_keyforms,
      position_indices,
      rotation_deformer_keyforms,
      uvs,
      warp_deformer_keyforms,
      keyform_multiply_colors,
      keyform_screen_colors,
      blend_shape_parameter_bindings,
      blend_shape_keyform_bindings,
      blend_shapes_warp_deformers,
      blend_shapes_art_deformers,
      blend_shape_constraint_indices,
      blend_shape_constraints,
      blend_shape_constraint_values,
      blend_shapes_parts,
      blend_shapes_rotation_deformers,
      blend_shapes_glue,
    })
  }
}
