pub mod xmlerror;
pub mod xmlutil;
pub enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
