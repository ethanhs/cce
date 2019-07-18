use reqwest::Client;

use crate::compiler::Compiler;
use crate::language::Language;
use crate::source::Output;

#[derive(Deserialize)]
struct Url {
    url: String,
}

pub fn get_languages(client: Client, host: &str) -> Vec<Language> {
    client
        .get(format!("{}/api/languages", host).as_str())
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

    let output: Output = client
        .post(format!("{}/api/compiler/{}/compile", host, &compiler).as_str())
        .json(&source)
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
    res
}

/// Send data to Compiler Explorer and shortens it. This may be used when the to be compiled sources are too large to fit into the URL.
/// Returns the shortened URL
pub fn shorten(client: Client, host: &str, src: String, compiler: &str, args: String) -> String {
    // Find language based on compiler.
    let compilers = get_compilers(&client.clone(), host, None);
    let language: String = compilers
        .iter()
        .find(|&x| x.id == compiler)
        .map_or("c++".to_string(), |c| c.lang.clone());

    let source = json!(
        { "sessions": [
            {
                "id": 1,
                "language": language,
                "source": src,
                "compilers": [{"id": &compiler,"options": args}]
            }
        ]}
    );

    let response = client
        .post(format!("{}/shortener", host).as_str())
        .json(&source)
        .send();

    let mut output_posted = match response {
        Ok(output_posted) => output_posted,
        Err(e) => return format!("Error sending: {}", e),
    };

    let output: Url = match output_posted.json() {
        Ok(output) => output,
        Err(e) => {
            return format!(
                "Error decoding result: {} {}",
                e,
                output_posted.text().unwrap()
            )
        }
    };

    return output.url;
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;

    fn setup_client() -> reqwest::Client {
        let mut headers = HeaderMap::new();
        headers.insert("ACCEPT", "application/json".parse().unwrap());
        headers.insert("ContentType", "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        return client;
    }

    #[test]
    fn test_shorten() {
        let client = setup_client();
        let src: String = "int main() { return 0; }".to_string();
        let result = shorten(client, "https://godbolt.org", src, "g91", "-O1".to_string());
        assert_eq!(result, "https://godbolt.org/z/CJ1Nvy".to_string());
    }

}
