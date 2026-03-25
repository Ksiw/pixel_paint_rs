use crate::domain::PaintDocument;

pub fn serialize_document(document: &PaintDocument) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(document)
}

pub fn deserialize_document(text: &str) -> Result<PaintDocument, serde_json::Error> {
    serde_json::from_str(text)
}
