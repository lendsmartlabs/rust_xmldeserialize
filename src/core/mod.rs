pub mod xmlerror;
pub mod xmlutil;
enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
