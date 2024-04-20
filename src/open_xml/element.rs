pub mod presentation;
use std::{fs, io::{self, Write}};

pub struct Types {
  tag: String,
  xmlns: XmlNamespace,
}

pub struct Relationships {
  tag: String,
  xmlns: XmlNamespace,
}

impl Types {
  pub fn new() -> Types {
    Types {
      tag: "Types".to_string(),
      xmlns: XmlNamespace::ContentTypes,
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    write_head_tag(writer, &self.tag, &self.xmlns)?;

    write_tail_tag(writer, &self.tag)?;

    Ok(())
  }
}

impl Relationships {
  pub fn new() -> Relationships {
    Relationships {
      tag: "Relationships".to_string(),
      xmlns: XmlNamespace::Relationships,
    }
  }

  pub fn write(&self, writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    write_head_tag(writer, &self.tag, &self.xmlns)?;
    write_tail_tag(writer, &self.tag)?;

    Ok(())
  }
}

fn write_head_tag(writer: &mut io::BufWriter<fs::File>, tag: &str, xmlns: &XmlNamespace) -> Result<(), io::Error> {
  let head = String::new() + "<" + tag;
  let head = head + " xmlns=\"" + &get_xmlns_string(xmlns) + "\"";
  let head = head + ">";
  writer.write_all(head.as_bytes())?;

  Ok(())
}

fn write_tail_tag(writer: &mut io::BufWriter<fs::File>, tag: &str) -> Result<(), io::Error> {
  let tail = String::new() + "</" + tag + ">";
  writer.write_all(tail.as_bytes())?;

  Ok(())
}

enum XmlNamespace {
  ContentTypes,
  Presentation,
  Relationships,
}

fn get_xmlns_string(xmlns: &XmlNamespace) -> String {
  match xmlns {
    XmlNamespace::ContentTypes => "http://schemas.openxmlformats.org/package/2006/content-types",
    XmlNamespace::Presentation => "http://schemas.openxmlformats.org/presentationml/2006/main",
    XmlNamespace::Relationships => "http://schemas.openxmlformats.org/package/2006/relationships",
  }.to_string()
}
