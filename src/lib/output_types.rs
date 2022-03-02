use serde::Serialize;


pub struct Output(pub Vec<OutputRow>);

#[derive(Serialize)]
pub struct OutputRow {
    pub client: u16,
    pub available: String,
    pub held: String,
    pub total: String,
    pub locked: bool
}

impl IntoIterator for Output {
    type Item = OutputRow;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}