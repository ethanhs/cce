#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

use clap::{App, AppSettings, Arg, SubCommand};

mod compiler;
mod language;
mod requests;
mod source;
mod tempedit;
mod url;

use requests::{compile, get_compilers, get_languages};
use reqwest::header::HeaderMap;
use tempedit::{edit_snippet, read_src};
use url::get_url;

fn main() {
    let matches = App::new("cce - a command line interface to compiler explorer")
        .version("0.1.0")
        .author("Ethan Smith")
        .about("Input C++, C, Rust, Haskell, Swift, etc, get assembly")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("host")
                .long("host")
                .takes_value(true)
                .default_value("https://godbolt.org")
                .help(" specify the Compiler Explorer host"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List the compilers and languages available on compiler explorer")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("langs").about(" list available languages."))
                .subcommand(
                    SubCommand::with_name("compilers")
                        .about(" list available compilers")
                        .arg(
                            Arg::with_name("language")
                                .short("l")
                                .long("lang")
                                .takes_value(true)
                                .help(" language to filter by"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("compile")
                .about("Compile a snippet on compiler explorer")
                .arg(
                    Arg::with_name("url")
                        .long("url")
                        .help("get an URL for given compilation"),
                )
                .arg(
                    Arg::with_name("id")
                        .takes_value(true)
                        .help(" compiler id to use for compilation")
                        .required(true),
                )
                .arg(
                    Arg::with_name("file")
                        .takes_value(true)
                        .help(" compile from the given source file"),
                )
                .arg(
                    Arg::with_name("args")
                        .multiple(true)
                        .allow_hyphen_values(true)
                        .help(" arguments to pass to the compiler"),
                ),
        )
        .get_matches();
    let mut headers = HeaderMap::new();
    headers.insert("ACCEPT", "application/json".parse().unwrap());
    headers.insert("ContentType", "application/json".parse().unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let host = matches.value_of("host").unwrap();

    if let Some(matches) = matches.subcommand_matches("list") {
        if let Some(_matches) = matches.subcommand_matches("langs") {
            let langs = get_languages(client, &host);

            for lang in langs {
                println!("{}", lang.id);
            }
        } else if let Some(matches) = matches.subcommand_matches("compilers") {
            let compilers = get_compilers(&client, &host, matches.value_of("language"));
            for compiler in compilers {
                println!("{}", compiler);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let all_compilers = get_compilers(&client, &host, None);
        let compiler = matches.value_of("id").unwrap();
        let valid_id = all_compilers.iter().any(|c| c.id == compiler);
        if !valid_id {
            println!("Not a valid compiler id! Run cce list compilers [lang]");
            std::process::exit(1)
        }
        let src = match matches.value_of("file") {
            Some(path) => read_src(path),
            None => edit_snippet(),
        };
        let args = matches.value_of("args").unwrap_or("").to_string();
        if matches.is_present("url") {
            let url = get_url(&src, &host, &compiler, &args);
            println!("URL: {}", url);
        }
        let asm = compile(client, &host, src, compiler, args);
        println!("{}", asm);
    }
}
