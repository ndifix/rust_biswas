mod presentation;

use std::io;
use std::fs;
use std::path;

use crate::open_xml::xml_file;

pub struct Document {
  working_dir: String,
  rels_dir: String,
  rels: xml_file::DocumentRelationships,
  presentation: presentation::Presentation,
  content_types: xml_file::ContentTypes,
}

impl Document {
  pub fn new() -> Document {
    let working_dir = String::from("tmp");
    let rels_dir = working_dir.clone() + "/_rels";

    Document {
      presentation: presentation::Presentation::new(&working_dir),
      content_types: xml_file::ContentTypes::new(working_dir.clone() + "/[Content_Types].xml"),
      working_dir,
      rels: xml_file::DocumentRelationships::new(rels_dir.clone() + "/.rels"),
      rels_dir,
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let path = path::Path::new(&self.working_dir);
    if path.is_dir() {
      let error = io::Error::new(io::ErrorKind::AlreadyExists, "working dir already exists");
      return Err(error);
    }

    fs::create_dir(&self.working_dir)?;

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
