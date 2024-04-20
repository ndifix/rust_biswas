use std::io::{self, Write};

mod xml_element;

pub struct XmlFile {
  path: String,
  root_element: xml_element::XmlRootElement,
}

impl XmlFile {
  pub fn new(path: String, tag: &str) -> XmlFile {
    XmlFile {
      path,
      root_element: xml_element::XmlRootElement::new(tag),
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
