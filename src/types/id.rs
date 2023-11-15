use crate::prelude::{Reading, Result};

#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct ID {
  pub value: String,
}

impl std::fmt::Debug for ID {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("ID({:#?})", self.value))
  }
}

impl<T> Reading<ID> for T
where
  T: Reading<u8>,
{
  const SIZE: usize = <T as Reading<u8>>::SIZE * 64;

  fn read_one(&mut self) -> Result<ID> {
    let bytes = self.read::<64>()?.to_vec();
    let value = String::from_utf8(bytes)?.trim_end_matches('\0').to_owned();

    Ok(ID { value })
  }
}
