use std::{fs::File, io::{self, Write}};

pub mod xml_file;

struct XmlElement {
  tag: String,
}

impl XmlElement {
  fn new(tag: &str) -> XmlElement {
    XmlElement {
      tag: tag.to_string(),
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<File>) -> Result<(), io::Error> {
    let head = "<".to_string() + &self.tag + ">";
    writer.write_all(head.as_bytes())?;

    let tail = "</".to_string() + &self.tag + ">";
    writer.write_all(tail.as_bytes())?;

    Ok(())
  }
}
