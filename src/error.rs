#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Bad Magic in MOC3 Header")]
  BadMagic,
  #[error(transparent)]
  FromUtf8(#[from] std::string::FromUtf8Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  TryFromInt(#[from] std::num::TryFromIntError),
  #[error("unknown deformer type: {0}")]
  UnknownDeformerType(u32),
  #[error("unknown draw order group object type: {0}")]
  UnknownDrawOrderGroupObjectType(u32),
}
