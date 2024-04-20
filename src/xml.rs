use std::io;

pub trait XmlFile {
  fn write(&self) -> Result<(), io::Error>;
}
