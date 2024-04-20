use std::io;

pub mod xml_element;

pub trait XmlFile {
  fn write(&self) -> Result<(), io::Error>;
}
