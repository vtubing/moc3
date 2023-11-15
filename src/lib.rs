pub(crate) mod prelude;
use prelude::*;

mod canvas;
mod error;
mod header;
mod reader;
pub mod types;

pub mod v3_00_00;
pub mod v3_03_00;
pub mod v4_00_00;
pub mod v4_02_00;
pub mod v5_00_00;

pub use canvas::{CanvasFlags, CanvasInfo};
pub use error::Error;
pub use header::{Header, Version};

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! NewType {
  ($Outer:ident, $Inner:ty) => {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub(crate) struct $Outer($Inner);

    impl From<$Inner> for $Outer {
      fn from(inner: $Inner) -> $Outer {
        $Outer(inner)
      }
    }

    impl From<$Outer> for $Inner {
      fn from(outer: $Outer) -> $Inner {
        outer.0
      }
    }

    impl AsRef<$Inner> for $Outer {
      fn as_ref(&self) -> &$Inner {
        &self.0
      }
    }
  };
}

pub(crate) use NewType;

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Model {
  V3_00_00(v3_00_00::Model),
  V3_03_00(v3_03_00::Model),
  V4_00_00(v4_00_00::Model),
  V4_02_00(v4_02_00::Model),
  V5_00_00(v5_00_00::Model),
}

impl Model {
  pub fn read(moc3: Vec<u8>) -> Result<Self> {
    let mut moc3 = Reader::try_from(moc3)?;

    info!("Model is {} bytes", moc3.len()?);

    let header: Header = moc3.read_one()?;

    moc3.rewind()?;

    let model = match header.version {
      Version::V3_00_00 | Version::Unknown => Self::V3_00_00(v3_00_00::Model::read(&mut moc3)?),
      Version::V3_03_00 => Self::V3_03_00(v3_03_00::Model::read(&mut moc3)?),
      Version::V4_00_00 => Self::V4_00_00(v4_00_00::Model::read(&mut moc3)?),
      Version::V4_02_00 => Self::V4_02_00(v4_02_00::Model::read(&mut moc3)?),
      Version::V5_00_00 => Self::V5_00_00(v5_00_00::Model::read(&mut moc3)?),
    };

    Ok(model)
  }
}

mod raw {
  crate::NewType!(Header, [u8; 64]);
  crate::NewType!(Magic, [u8; 4]);
  crate::NewType!(Version, u8);
  crate::NewType!(Endianness, u8);
  crate::NewType!(Padding, [u8; 58]);

  impl From<&Header> for Magic {
    fn from(header: &Header) -> Self {
      let header = header.as_ref();
      [header[0], header[1], header[2], header[3]].into()
    }
  }

  impl From<&Header> for Version {
    fn from(header: &Header) -> Self {
      let header = header.as_ref();
      header[4].into()
    }
  }

  impl From<&Header> for Endianness {
    fn from(header: &Header) -> Self {
      let header = header.as_ref();
      header[5].into()
    }
  }

  impl From<&Header> for Padding {
    fn from(header: &Header) -> Self {
      let header = header.as_ref();
      [
        header[6], header[7], header[8], header[9], header[10], header[11], header[12], header[13], header[14], header[15], header[16], header[17], header[18], header[19], header[20], header[21],
        header[22], header[23], header[24], header[25], header[26], header[27], header[28], header[29], header[30], header[31], header[32], header[33], header[34], header[35], header[36], header[37],
        header[38], header[39], header[40], header[41], header[42], header[43], header[44], header[45], header[46], header[47], header[48], header[49], header[50], header[51], header[52], header[53],
        header[54], header[55], header[56], header[57], header[58], header[59], header[60], header[61], header[62], header[63],
      ]
      .into()
    }
  }
}
