mod bit_field;
mod bool32;
mod id;
mod offset;
mod padding;

pub use bit_field::BitField;
pub use bool32::Bool32;
pub use id::ID;
pub use offset::Offset;
pub use padding::Padding;

// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// struct RawHeader(pub [u8; 64]);

// impl Default for RawHeader {
//   fn default() -> Self {
//     Self([0u8; 64])
//   }
// }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct RawSectionOffsetTable(pub [u8; 640]);

impl Default for RawSectionOffsetTable {
  fn default() -> Self {
    Self([0u8; 640])
  }
}
