use std::env;
use std::fs::{create_dir, File};
use std::io::Read;
use std::process::Command;

use dirs;

#[cfg(unix)]
fn default_editor() -> String {
    String::from("vim")
}
#[cfg(target_os = "windows")]
fn default_editor() -> String {
    String::from("notepad")
}

pub fn edit_snippet() -> String {
    let home = dirs::home_dir().expect("Fatal: user needs home directory");
    let home_dir = home.as_path().join(".godboltc");
    let godboltc = home_dir.as_path();
    if !godboltc.exists() || !godboltc.is_dir() {
        create_dir(&godboltc).expect("Unable to create ~/.godboltc");
    }
    let main = godboltc.join("main");
    let godboltc = main.as_path();
    if !godboltc.exists() || !godboltc.is_file() {
        File::create(&godboltc)
            .unwrap_or_else(|_| panic!("Could not create file {}", godboltc.to_str().unwrap()));
    }
    let mut temp = File::open(&godboltc).expect("Unable to create temp file");
    let editor = match env::var("VISUAL").ok() {
        Some(edit) => edit,
        None => match env::var("EDITOR").ok() {
            Some(edit) => edit,
            None => default_editor(),
        },
    };

    Command::new(editor)
        .arg(godboltc.to_str().unwrap())
        .status()
        .expect("Failed to open editor");

    let mut buf = String::new();
    temp.read_to_string(&mut buf).unwrap();
    buf
}

pub fn read_src(path: &str) -> String {
    let mut f = File::open(path).unwrap_or_else(|_| panic!("could not find file {}", path));
    let mut src = String::new();
    f.read_to_string(&mut src).expect("Failed to read file");
    src
}
