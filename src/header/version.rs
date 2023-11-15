#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum Version {
  #[default]
  /// unknown
  Unknown = 0,
  /// 3.0.00 - 3.2.07
  V3_00_00 = 1,
  /// 3.3.00 - 3.3.03
  V3_03_00 = 2,
  /// 4.0.00 - 4.1.05
  V4_00_00 = 3,
  /// 4.2.00 - 4.2.02
  V4_02_00 = 4,
  /// 5.0.00 -
  V5_00_00 = 5,
}

impl From<u8> for Version {
  fn from(byte: u8) -> Self {
    match byte {
      1 => Self::V3_00_00,
      2 => Self::V3_03_00,
      3 => Self::V4_00_00,
      4 => Self::V4_02_00,
      5 => Self::V5_00_00,
      _ => Self::Unknown,
    }
  }
}
