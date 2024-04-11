mod document;
mod xml;

pub fn build_document() {
  let document = crate::document::Document::new();
  if let Err(e) = document.write() {
    println!("Failed to write files: {}", e);
  }
}
