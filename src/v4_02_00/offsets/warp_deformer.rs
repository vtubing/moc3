use super::*;
use v4_00_00::offsets as v4_00_00;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WarpDeformerOffsets {
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub vertex_counts: Offset,
  pub rows: Offset,
  pub columns: Offset,
  pub is_quad_source: Offset,
  pub keyform_color_sources_begin_indices: Offset,
}

impl From<[Offset; 8]> for WarpDeformerOffsets {
  fn from(
    [keyform_binding_sources_indices, keyform_sources_begin_indices, keyform_sources_counts, vertex_counts, rows, columns, is_quad_source, keyform_color_sources_begin_indices]: [Offset; 8],
  ) -> Self {
    Self {
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      vertex_counts,
      rows,
      columns,
      is_quad_source,
      keyform_color_sources_begin_indices,
    }
  }
}

impl From<(v4_00_00::WarpDeformerOffsets, [Offset; 1])> for WarpDeformerOffsets {
  fn from(
    (
      v4_00_00::WarpDeformerOffsets {
        keyform_binding_sources_indices,
        keyform_sources_begin_indices,
        keyform_sources_counts,
        vertex_counts,
        rows,
        columns,
        is_quad_source,
      },
      [keyform_color_sources_begin_indices],
    ): (v4_00_00::WarpDeformerOffsets, [Offset; 1]),
  ) -> Self {
    [
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      vertex_counts,
      rows,
      columns,
      is_quad_source,
      keyform_color_sources_begin_indices,
    ]
    .into()
  }
}

impl<T> Reading<WarpDeformerOffsets> for T
where
  T: Reading<Offset>,
{
  fn read_one(&mut self) -> Result<WarpDeformerOffsets> {
    let offsets = self.read()?;
    Ok(offsets.into())
  }
}
