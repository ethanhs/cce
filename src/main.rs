#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate hyper;
extern crate reqwest;

use clap::{App, AppSettings, Arg, SubCommand};
use hyper::header::{qitem, Accept, Headers};
use hyper::mime;

mod compiler;
mod language;
mod requests;
mod source;
mod tempedit;

use requests::{compile, get_compilers, get_languages};
use tempedit::edit_snippet;

fn main() {
    let matches = App::new("cce - a command line interface to compiler explorer")
        .version("0.1.0")
        .author("Ethan Smith")
        .about("Input C++, C, Rust, Haskell, Swift, etc, get assembly")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("List the compilers and languages available on compiler explorer")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("langs").about(" list available languages."))
                .subcommand(
                    SubCommand::with_name("compilers")
                        .about(" list avaiable compilers")
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
                    Arg::with_name("id")
                        .takes_value(true)
                        .help(" compiler id to use for compilation")
                        .required(true),
                )
                .arg(
                    Arg::with_name("args")
                        .takes_value(true)
                        .help(" arguments to pass to the compiler"),
                ),
        )
        .get_matches();
    let mut headers = Headers::new();
    headers.set(Accept(vec![qitem(mime::APPLICATION_JSON)]));
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    if let Some(matches) = matches.subcommand_matches("list") {
        if let Some(_matches) = matches.subcommand_matches("langs") {
            let langs = get_languages(client);
            for lang in langs {
                println!("{}", lang.id);
            }
        } else if let Some(matches) = matches.subcommand_matches("compilers") {
            let compilers = get_compilers(client, matches.value_of("language"));
            for compiler in compilers {
                println!("{}", compiler);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let src = edit_snippet();
        let compiler = matches.value_of("id").unwrap();
        let args = matches.value_of("id").unwrap_or("").to_string();
        let asm = compile(client, src, compiler, args);
        println!("Compiling with {} compiler outputs:", compiler);
        println!("");
        println!("{}", asm);
    }
}
