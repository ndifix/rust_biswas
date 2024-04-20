use std::{fs, io::{self, Write}};

pub struct XmlRootElement {
  tag: String,
}

impl XmlRootElement {
  pub fn new(tag: &str) -> XmlRootElement {
    XmlRootElement {
      tag: tag.to_string(),
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    let head = "<".to_string() + &self.tag + ">";
    writer.write_all(head.as_bytes())?;

    let tail = "</".to_string() + &self.tag + ">";
    writer.write_all(tail.as_bytes())?;

    Ok(())
  }
}
