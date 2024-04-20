mod presentation;

use std::io;
use std::fs;
use std::path;

use crate::xml;

pub struct Document {
  tmp_dir: String,
  presemtation: presentation::Presentation,
  content_types: xml::XmlFile,
}

impl Document {
  pub fn new() -> Document {
    let tmp_dir = String::from("tmp");

    let content_types = xml::XmlFile::new(tmp_dir.clone() + "/[Content_Types].xml", "Types");

    Document {
      presemtation: presentation::Presentation::new(&tmp_dir),
      tmp_dir,
      content_types,
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let path = path::Path::new(&self.tmp_dir);
    if path.is_dir() {
      let error = io::Error::new(io::ErrorKind::AlreadyExists, "working dir already exists");
      return Err(error);
    }

    fs::create_dir(&self.tmp_dir)?;

    if let Err(e) = self.content_types.write() {
      return Err(e);
    }

    if let Err(e) = self.presemtation.write() {
      return Err(e);
    }

    Ok(())
  }
}
