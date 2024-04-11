use std::{fs, io};

pub struct Theme {
  part_dir: String,
}

impl Theme {
  pub fn new(ppt_path: &str) -> Theme {
    let part_dir = String::new() + ppt_path + "/theme";

    Theme {
      part_dir,
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    fs::create_dir(&self.part_dir)?;

    Ok(())
  }
}
