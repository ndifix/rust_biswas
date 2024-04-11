use std::{fs, io};

use crate::xml::xml_file;


pub struct Presentation {
  part_dir: String,
  xml_file: xml_file::XmlFile,
}

impl Presentation {
  pub fn new(doc_path: &str) -> Presentation {
    let part_dir = String::new() + doc_path + "/ppt";
    let xml_file = xml_file::XmlFile::new(part_dir.clone() + "/presentation.xml");

    Presentation {
      part_dir,
      xml_file,
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    fs::create_dir(&self.part_dir)?;

    self.xml_file.write()?;

    Ok(())
  }
}
