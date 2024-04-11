mod document;

pub fn build_document() {
  let document = crate::document::Document::new();
  document.write();
}
