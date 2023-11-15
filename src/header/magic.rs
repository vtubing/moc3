#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Magic([u8; 4]);

impl From<[u8; 4]> for Magic {
  fn from(bytes: [u8; 4]) -> Self {
    Self(bytes)
  }
}

impl std::fmt::Debug for Magic {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_valid() {
      f.write_fmt(format_args!("Magic({:?}) - VALID", self.0))
    } else {
      f.write_fmt(format_args!("Magic({:?}) - INVALID", self.0))
    }
  }
}

const MAGIC: [u8; 4] = [77, 79, 67, 51]; // MOC3

impl Magic {
  pub fn is_valid(&self) -> bool {
    self.0 == MAGIC
  }
}
