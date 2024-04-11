
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
