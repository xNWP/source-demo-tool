#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct EventData {
    pub details: String,
    pub line_number: u32,
    pub function_name: String,
    pub file_name: String
}