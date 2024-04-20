use std::io::{self, Write};
use super::element::{self, presentation};

pub struct Presentation {
  path: String,
  root_element: presentation::Presentation,
}

pub struct ContentTypes {
  path: String,
  root_element: element::Types,
}

impl Presentation {
  pub fn new(path: String) -> Presentation {
    Presentation {
      path,
      root_element: presentation::Presentation::new(),
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let file = std::fs::File::create(&self.path)?;

    let mut writer = io::BufWriter::new(file);
    self.root_element.write(&mut writer)?;
    writer.flush()?;

    Ok(())
  }
}

impl ContentTypes {
  pub fn new(path: String) -> ContentTypes {
    ContentTypes {
      path,
      root_element: super::element::Types::new(),
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let file = std::fs::File::create(&self.path)?;

    let mut writer = io::BufWriter::new(file);
    self.root_element.write(&mut writer)?;
    writer.flush()?;

    Ok(())
  }
}
