mod presentation;

use std::io;
use std::fs;
use std::path;

use crate::open_xml::xml_file;

pub struct Document {
  tmp_dir: String,
  presemtation: presentation::Presentation,
  content_types: xml_file::ContentTypes,
}

impl Document {
  pub fn new() -> Document {
    let tmp_dir = String::from("tmp");

    Document {
      presemtation: presentation::Presentation::new(&tmp_dir),
      content_types: xml_file::ContentTypes::new(tmp_dir.clone() + "/[Content_Types].xml"),
      tmp_dir,
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
