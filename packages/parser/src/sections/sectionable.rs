pub trait Sectionable {
    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Self;
}
