#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub idx: i32,
    pub id: i32,
    pub str: String,
    pub name: String
}