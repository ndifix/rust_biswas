use std::io::{self, Write};

use super::XmlElement;


pub struct XmlFile {
  path: String,
  root_element: XmlElement,
}

impl XmlFile {
  pub fn new(path: String, tag: &str) -> XmlFile {
    XmlFile {
      path,
      root_element: XmlElement::new(tag),
    }
  }

  pub fn write(&self) -> Result<(), io::Error>{
    let file = std::fs::File::create(&self.path)?;

    let mut writer = io::BufWriter::new(file);
    self.root_element.write(&mut writer)?;
    writer.flush()?;

    Ok(())
  }
}
