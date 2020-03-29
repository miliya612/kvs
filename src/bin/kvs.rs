#[macro_use]
extern crate clap;
use clap::App;

use std::env;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    match matches.subcommand() {
        ("get", Some(_)) => panic!("unimplemented"),
        ("set", Some(_)) => panic!("unimplemented"),
        ("rm", Some(_)) => panic!("unimplemented"),
        ("", None) => panic!("No subcommand was specified!"),
        _ => unreachable!(),
    }
}
