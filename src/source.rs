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
