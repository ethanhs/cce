use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub id: String,
    pub name: String,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.id)
    }
}
