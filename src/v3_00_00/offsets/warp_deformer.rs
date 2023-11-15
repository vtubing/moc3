use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WarpDeformerOffsets {
  pub keyform_binding_sources_indices: Offset,
  pub keyform_sources_begin_indices: Offset,
  pub keyform_sources_counts: Offset,
  pub vertex_counts: Offset,
  pub rows: Offset,
  pub columns: Offset,
}

impl From<[Offset; 6]> for WarpDeformerOffsets {
  fn from([keyform_binding_sources_indices, keyform_sources_begin_indices, keyform_sources_counts, vertex_counts, rows, columns]: [Offset; 6]) -> Self {
    Self {
      keyform_binding_sources_indices,
      keyform_sources_begin_indices,
      keyform_sources_counts,
      vertex_counts,
      rows,
      columns,
    }
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
