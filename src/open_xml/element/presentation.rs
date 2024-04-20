use std::{fs, io};

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
    super::write_head_tag(writer, &self.tag)?;

    super::write_tail_tag(writer, &self.tag)?;

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
    super::write_head_tag(writer, &self.tag)?;

    super::write_tail_tag(writer, &self.tag)?;

    Ok(())
  }
}
