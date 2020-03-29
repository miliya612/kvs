#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(_) = matches.subcommand_name() {
    } else {
        panic!("No subcommand was used")
    }
}
