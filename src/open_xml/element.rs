pub mod presentation;
use std::{fs, io::{self, Write}};

pub struct Types {
  tag: String,
}

pub struct Relationships {
  tag: String,
}

impl Types {
  pub fn new() -> Types {
    Types {
      tag: "Types".to_string(),
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    write_head_tag(writer, &self.tag)?;

    write_tail_tag(writer, &self.tag)?;

    Ok(())
  }
}

impl Relationships {
  pub fn new() -> Relationships {
    Relationships {
      tag: "Relationships".to_string(),
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    write_head_tag(writer, &self.tag)?;
    write_tail_tag(writer, &self.tag)?;

    Ok(())
  }
}

fn write_head_tag(writer: &mut io::BufWriter<fs::File>, tag: &str) -> Result<(), io::Error> {
  let head = String::new() + "<" + tag + ">";
  writer.write_all(head.as_bytes())?;

  Ok(())
}

fn write_tail_tag(writer: &mut io::BufWriter<fs::File>, tag: &str) -> Result<(), io::Error> {
  let tail = String::new() + "</" + tag + ">";
  writer.write_all(tail.as_bytes())?;

  Ok(())
}
