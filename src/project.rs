#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}
