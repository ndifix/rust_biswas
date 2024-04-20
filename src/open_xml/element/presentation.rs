use std::{fs, io};

use super::XmlNamespace;

pub struct Presentation {
  tag: String,
  xmlns: XmlNamespace,
}

pub struct PresentationProperties {
  tag: String,
  xmlns: XmlNamespace,
}

impl Presentation {
  pub fn new() -> Presentation {
    Presentation {
      tag: "presentation".to_string(),
      xmlns: XmlNamespace::Presentation,
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    super::write_head_tag(writer, &self.tag, &self.xmlns)?;

    super::write_tail_tag(writer, &self.tag)?;

    Ok(())
  }
}

impl PresentationProperties {
  pub fn new() -> PresentationProperties {
    PresentationProperties {
      tag: "presentationPr".to_string(),
      xmlns: XmlNamespace::Presentation,
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    super::write_head_tag(writer, &self.tag, &self.xmlns)?;

    super::write_tail_tag(writer, &self.tag)?;

    Ok(())
  }
}
