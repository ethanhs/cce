use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Compiler {
    pub id: String,
    pub name: String,
    pub lang: String,
}

impl fmt::Display for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Id: {}", self.name, self.id)
    }
}
