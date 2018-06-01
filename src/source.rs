#[derive(Serialize, Deserialize, Debug)]
pub struct Filters {
    pub intel: bool,
    pub demangle: bool,
    pub directives: bool,
    pub comments: bool,
    pub labels: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub userArguments: String,
    pub filters: Filters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub source: String,
    pub options: Options,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asm {
    pub asm: Vec<Text>,
}
