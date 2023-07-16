pub trait Sectionable {
    type SelfType;

    fn from_section(properties: Vec<&str>, comment: Option<String>) -> Result<Self::SelfType, String>
    where
    Self: Sized;
}
