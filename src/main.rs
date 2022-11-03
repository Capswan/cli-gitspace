use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Arguments {
    #[clap(short, long)]
    config_file: Option<String>,

    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Init {},
    Sync {},
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Init {} => {
            println!("init");
        }
        SubCommand::Sync {} => {
            println!("sync");
        }
    }
}
