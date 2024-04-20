mod presentation;

use std::io;
use std::fs;
use std::path;

use crate::open_xml::xml_file;

pub struct Document {
  tmp_dir: String,
  rels_dir: String,
  rels: xml_file::DocumentRelationships,
  presentation: presentation::Presentation,
  content_types: xml_file::ContentTypes,
}

impl Document {
  pub fn new() -> Document {
    let tmp_dir = String::from("tmp");
    let rels_dir = tmp_dir.clone() + "/_rels";

    Document {
      presentation: presentation::Presentation::new(&tmp_dir),
      content_types: xml_file::ContentTypes::new(tmp_dir.clone() + "/[Content_Types].xml"),
      tmp_dir,
      rels: xml_file::DocumentRelationships::new(rels_dir.clone() + "/.rels"),
      rels_dir,
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

    if let Err(e) = self.presentation.write() {
      return Err(e);
    }

    fs::create_dir(&self.rels_dir)?;
    if let Err(e) = self.rels.write() {
      return Err(e);
    }

    Ok(())
  }
}
