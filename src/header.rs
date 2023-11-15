use crate::prelude::{Padding, Reading, Result};

mod magic;
mod version;

pub use magic::Magic;
pub use version::Version;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Header {
  pub magic: Magic,
  pub version: Version,
  pub is_big_endian: bool,
  pub padding: Padding<58>,
}

impl From<([u8; 4], u8, u8, [u8; 58])> for Header {
  fn from((magic, version, is_big_endian, padding): ([u8; 4], u8, u8, [u8; 58])) -> Self {
    let magic = Magic::from(magic);
    let version = Version::from(version);
    let is_big_endian = is_big_endian > 0;
    let padding = Padding::from(padding);

    Self {
      magic,
      version,
      is_big_endian,
      padding,
    }
  }
}

impl From<[u8; 64]> for Header {
  fn from(bytes: [u8; 64]) -> Self {
    let magic = [bytes[0], bytes[1], bytes[2], bytes[3]];
    let version = bytes[4];
    let is_big_endian = bytes[5];
    let padding = [
      bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15], bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22], bytes[23],
      bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29], bytes[30], bytes[31], bytes[32], bytes[33], bytes[34], bytes[35], bytes[36], bytes[37], bytes[38], bytes[39], bytes[40],
      bytes[41], bytes[42], bytes[43], bytes[44], bytes[45], bytes[46], bytes[47], bytes[48], bytes[49], bytes[50], bytes[51], bytes[52], bytes[53], bytes[54], bytes[55], bytes[56], bytes[57],
      bytes[58], bytes[59], bytes[60], bytes[61], bytes[62], bytes[63],
    ];

    (magic, version, is_big_endian, padding).into()
  }
}

impl<T> Reading<Header> for T
where
  T: Reading<u8>,
{
  fn read_one(&mut self) -> Result<Header> {
    let bytes = self.read()?;
    Ok(bytes.into())
  }
}
