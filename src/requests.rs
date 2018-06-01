use reqwest::Client;

use compiler::Compiler;
use language::Language;
use source::{Asm, Filters, Options, Source};

pub fn get_languages(client: Client) -> Vec<Language> {
    client
        .get("https://www.godbolt.org/api/languages")
        .send()
        .expect("Failed to commit transaction in get_languages")
        .json()
        .expect("Failed to parse JSON in get_languages")
}

pub fn get_compilers(client: Client, language: Option<&str>) -> Vec<Compiler> {
    match language {
        Some(lang) => client
            .get(format!("https://www.godbolt.org/api/compilers/{}", lang).as_str())
            .send()
            .unwrap()
            .json()
            .unwrap(),
        None => client
            .get("https://www.godbolt.org/api/compilers/")
            .send()
            .unwrap()
            .json()
            .unwrap(),
    }
}

pub fn compile(client: Client, src: String, compiler: &str, args: String) -> String {
    let filters = Filters {
        intel: true,
        demangle: true,
        directives: true,
        comments: true,
        labels: true,
    };
    let options = Options {
        userArguments: args,
        filters: filters,
    };
    let source = Source {
        source: src,
        options: options,
    };
    let asm: Asm = client
        .post(format!("https://www.godbolt.org/api/compiler/{}/compile", &compiler).as_str())
        .json(&source)
        .send()
        .unwrap()
        .json()
        .unwrap();
    let mut res = String::new();
    for line in asm.asm {
        res.push_str(&line.text);
        res.push('\n');
    }
    return res;
}
