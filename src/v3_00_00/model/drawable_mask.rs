use super::offsets::DrawableMaskOffsets;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct DrawableMask {
  pub art_mesh_sources_index: i32,
}

impl ExtractFromOffsets for DrawableMask {
  type Offsets = DrawableMaskOffsets;

  fn extract_one(index: u64, offsets: &Self::Offsets, moc3: &mut Reader) -> Result<Self> {
    Ok(Self {
      art_mesh_sources_index: moc3.read_one_at_offset_with_index(offsets.art_mesh_sources_indices, index)?,
    })
  }
}
