mod slide_master;
mod theme;

use std::{fs, io};

use crate::xml::xml_file;

pub struct Presentation {
  part_dir: String,
  xml_file: xml_file::XmlFile,
  theme: theme::Theme,
  slide_masters: slide_master::SlideMasters,
}

impl Presentation {
  pub fn new(doc_path: &str) -> Presentation {
    let part_dir = String::new() + doc_path + "/ppt";
    let xml_file = xml_file::XmlFile::new(part_dir.clone() + "/presentation.xml", "presentation");

    Presentation {
      theme: theme::Theme::new(&part_dir),
      slide_masters: slide_master::SlideMasters::new(&part_dir),
      part_dir,
      xml_file,
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    fs::create_dir(&self.part_dir)?;

    self.xml_file.write()?;
    self.theme.write()?;
    self.slide_masters.write()?;

    Ok(())
  }
}
