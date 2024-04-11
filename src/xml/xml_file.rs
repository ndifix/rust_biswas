use std::io;


pub struct XmlFile {
  path: String,
}

impl XmlFile {
  pub fn new(path: String) -> XmlFile {
    XmlFile {
      path,
    }
  }

  pub fn write(&self) -> Result<(), io::Error>{
    std::fs::File::create(&self.path)?;

    Ok(())
  }
}
