
mod document {
  pub struct Document {
    tmp_dir: String,
  }

  impl Document {
    pub fn new() -> Document {
      Document {
        tmp_dir: String::from("tmp"),
      }
    }

    pub fn write(&self) {
      println!("{}", self.tmp_dir);
    }
  }
}

pub fn build_document() {
  let document = crate::document::Document::new();
  document.write();
}
