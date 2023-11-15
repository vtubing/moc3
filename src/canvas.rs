use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct CanvasInfo {
  pub pixels_per_unit: f32,
  pub origin_x: f32,
  pub origin_y: f32,
  pub width: f32,
  pub height: f32,
  pub flags: CanvasFlags,
  pub padding: Padding<43>,
}

impl CanvasInfo {
  pub fn read(moc3: &mut Reader) -> Result<Self> {
    trace!("{:#010x} CanvasInfo", moc3.stream_position()?);

    let pixels_per_unit = moc3.read_one()?;
    let origin_x = moc3.read_one()?;
    let origin_y = moc3.read_one()?;
    let width = moc3.read_one()?;
    let height = moc3.read_one()?;
    let flags = moc3.read_one()?;
    let padding = moc3.read_one()?;

    Ok(Self {
      pixels_per_unit,
      origin_x,
      origin_y,
      width,
      height,
      flags,
      padding,
    })
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct CanvasFlags {
  pub reverse_y_coordinate: bool,
}

impl From<[bool; 1]> for CanvasFlags {
  fn from([reverse_y_coordinate]: [bool; 1]) -> Self {
    Self { reverse_y_coordinate }
  }
}

impl From<BitField> for CanvasFlags {
  fn from(BitField([reverse_y_coordinate, ..]): BitField) -> Self {
    [reverse_y_coordinate].into()
  }
}

impl<T> Reading<CanvasFlags> for T
where
  T: Reading<BitField>,
{
  fn read_one(&mut self) -> Result<CanvasFlags> {
    Ok(self.read_one()?.into())
  }
}
