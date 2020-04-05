use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(subcommand)]
    pub sub: Cmd,
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "set", about = "Set the value of a string key to a string")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Set {
        #[structopt(name = "KEY")]
        key: String,
        #[structopt(name = "VALUE")]
        value: String,
    },
    #[structopt(name = "get", about = "Get the string value of a given string key")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Get {
        #[structopt(name = "KEY")]
        key: String,
    },
    #[structopt(name = "rm", about = "Remove a given key")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Remove {
        #[structopt(name = "KEY")]
        key: String,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt.sub {
        Cmd::Get { key: _ } => panic!("unimplemented"),
        Cmd::Set { key: _, value: _ } => panic!("unimplemented"),
        Cmd::Remove { key: _ } => panic!("unimplemented"),
    }
}
