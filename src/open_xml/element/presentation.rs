use std::{fs, io::{self, Write}};

pub struct Presentation {
  tag: String,
}

pub struct PresentationProperties {
  tag: String,
}

impl Presentation {
  pub fn new() -> Presentation {
    Presentation {
      tag: "presentation".to_string(),
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

impl PresentationProperties {
  pub fn new() -> PresentationProperties {
    PresentationProperties {
      tag: "presentationPr".to_string(),
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
