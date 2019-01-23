use reqwest::Client;

use compiler::Compiler;
use language::Language;
use source::Output;

pub fn get_languages(client: Client, host: &str) -> Vec<Language> {
    client.get(format!("{}/api/languages", host).as_str())
        .send()
        .expect("Failed to commit transaction in get_languages")
        .json()
        .expect("Failed to parse JSON in get_languages")
}

pub fn get_compilers(client: &Client, host: &str, language: Option<&str>) -> Vec<Compiler> {
    match language {
        Some(lang) => client
            .get(format!("{}/api/compilers/{}", host, lang).as_str())
            .send()
            .unwrap()
            .json()
            .unwrap(),
        None => client
            .get(format!("{}/api/compilers/", host).as_str())
            .send()
            .unwrap()
            .json()
            .unwrap(),
    }
}

pub fn compile(client: Client, host: &str, src: String, compiler: &str, args: String) -> String {
    let filters = json!(
        {
            "intel": true,
            "demangle": true,
            "directives": true,
            "comments": true,
            "labels": true
        }
    );

    let options = json!({
        "userArguments": args,
        "filters": filters
    });

    let source = json!({
        "source": src,
        "options": options
    });

    let output : Output =  client
        .post(format!("{}/api/compiler/{}/compile", host, &compiler).as_str())
        .body(source.to_string())
        .send()
        .unwrap()
        .json()
        .unwrap();

    let mut res = String::new();
    if output.code != 0 {
        for line in output.stderr {
            res.push_str(&line.text);
            res.push('\n');
        }
    } else {
        for line in output.asm {
            res.push_str(&line.text);
            res.push('\n');
        }
    }
    return res;
}
