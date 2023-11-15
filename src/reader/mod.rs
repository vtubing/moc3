use crate::Result;

mod private;
mod public;
mod traits;

use private::ReaderForByteOrder;
pub(crate) use public::Reader;
pub(crate) use traits::{Reading, Seeking};

impl<T> Reading<bool> for T
where
  T: Reading<u8>,
{
  fn read_one(&mut self) -> Result<bool> {
    self.read_one().map(|value| value > 0)
  }
}
