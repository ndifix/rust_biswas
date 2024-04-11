use std::io;
use std::fs;
use std::path::Path;

pub struct Document {
  tmp_dir: String,
}

impl Document {
  pub fn new() -> Document {
    Document {
      tmp_dir: String::from("tmp"),
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let path = Path::new(&self.tmp_dir);
    if path.is_dir() {
      let error = io::Error::new(io::ErrorKind::AlreadyExists, "working dir already exists");
      return Err(error);
    }

    fs::create_dir(&self.tmp_dir)?;

    Ok(())
  }
}
