use super::v3_03_00;

pub use v3_03_00::CountInfoTable;

pub mod offsets {
  pub use super::v3_03_00::offsets::{
    ArtMeshKeyformOffsets, ArtMeshOffsets, DeformerOffsets, DrawOrderGroupObjectOffsets, DrawOrderGroupOffsets, DrawableMaskOffsets, GlueInfoOffsets, GlueKeyformOffsets, GlueOffsets, KeyOffsets,
    KeyformBindingOffsets, KeyformPositionOffsets, ParameterBindingIndicesOffsets, ParameterBindingOffsets, ParameterOffsets, PartKeyformOffsets, PartOffsets, PositionIndicesOffsets,
    RotationDeformerKeyformOffsets, RotationDeformerOffsets, SectionOffsetTable, UvOffsets, WarpDeformerKeyformOffsets, WarpDeformerOffsets,
  };
}

pub mod model {
  pub use super::v3_03_00::model::{
    ArtMesh, ArtMeshDrawableFlags, ArtMeshKeyform, Deformer, DeformerType, DrawOrderGroup, DrawOrderGroupObject, DrawOrderGroupObjectType, DrawableMask, Glue, GlueInfo, GlueKeyform, Key,
    KeyformBinding, KeyformPosition, Model, Parameter, ParameterBinding, ParameterBindingIndices, Part, PartKeyform, PositionIndices, RotationDeformer, RotationDeformerKeyform, Uv, WarpDeformer,
    WarpDeformerKeyform,
  };
}

pub use model::Model;
pub use offsets::SectionOffsetTable;
