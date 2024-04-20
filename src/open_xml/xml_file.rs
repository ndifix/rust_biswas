use std::io::{self, Write};
use crate::xml::{self, xml_element};

pub struct Presentation {
  path: String,
  root_element: xml_element::XmlRootElement,
}

pub struct ContentTypes {
  path: String,
  root_element: xml_element::XmlRootElement,
}

impl Presentation {
  pub fn new(path: String) -> Presentation {
    Presentation {
      path,
      root_element: xml_element::XmlRootElement::new("presentation"),
    }
  }
}

impl ContentTypes {
  pub fn new(path: String) -> ContentTypes {
    ContentTypes {
      path,
      root_element: xml_element::XmlRootElement::new("Types"),
    }
  }
}

impl xml::XmlFile for Presentation {
  fn write(&self) -> Result<(), io::Error> {
    let file = std::fs::File::create(&self.path)?;

    let mut writer = io::BufWriter::new(file);
    self.root_element.write(&mut writer)?;
    writer.flush()?;

    Ok(())
  }
}

impl xml::XmlFile for ContentTypes {
  fn write(&self) -> Result<(), io::Error> {
    let file = std::fs::File::create(&self.path)?;

    let mut writer = io::BufWriter::new(file);
    self.root_element.write(&mut writer)?;
    writer.flush()?;

    Ok(())
  }
}
