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
    // This is needed to workaround #955 in compiler-explorer where it
    // may return objects without text field.
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub code: i32,
    pub stderr: Vec<Text>,
    pub asm: Vec<Text>,
}
