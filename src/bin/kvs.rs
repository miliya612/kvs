#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("get", Some(_)) => panic!("unimplemented"),
        ("set", Some(_)) => panic!("unimplemented"),
        ("rm", Some(_)) => panic!("unimplemented"),
        ("", None) => panic!("No subcommand was specified!"),
        _ => unreachable!(),
    }
}
