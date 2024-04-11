use std::{fs, io};

pub struct SlideMasters {
  part_dir: String,
}

impl SlideMasters {
  pub fn new(ppt_path: &str) -> SlideMasters {
    let part_dir = String::new() + ppt_path + "/slideMasters";

    SlideMasters {
      part_dir,
    }
  }

  pub fn write(&self) -> Result<(), io::Error> {
    fs::create_dir(&self.part_dir)?;

    Ok(())
  }
}
