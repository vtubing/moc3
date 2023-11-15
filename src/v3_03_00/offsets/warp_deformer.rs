use super::*;
use v3_00_00::offsets as v3_00_00;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WarpDeformerOffsets {
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub vertex_counts: Offset,
  pub rows: Offset,
  pub columns: Offset,
  pub is_quad_source: Offset,
}

impl From<[Offset; 7]> for WarpDeformerOffsets {
  fn from([keyform_binding_sources_indices, keyform_sources_begin_indices, keyform_sources_counts, vertex_counts, rows, columns, is_quad_source]: [Offset; 7]) -> Self {
    Self {
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      vertex_counts,
      rows,
      columns,
      is_quad_source,
    }
  }
}

impl From<(v3_00_00::WarpDeformerOffsets, [Offset; 1])> for WarpDeformerOffsets {
  fn from(
    (
      v3_00_00::WarpDeformerOffsets {
        keyform_binding_sources_indices,
        keyform_sources_begin_indices,
        keyform_sources_counts,
        vertex_counts,
        rows,
        columns,
      },
      [is_quad_source],
    ): (v3_00_00::WarpDeformerOffsets, [Offset; 1]),
  ) -> Self {
    [
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      vertex_counts,
      rows,
      columns,
      is_quad_source,
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
