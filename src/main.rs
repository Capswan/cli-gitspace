//! Gitspace
// use std::io::prelude::*;
use clap::{Parser, Subcommand};
mod config;
use config::{Config, ConfigFile, ConfigParser, Paths};
use std::path::Path;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Arguments {
    #[clap(short, long)]
    config_file: Option<String>,

    #[clap(short, long)]
    ssh_key: Option<String>,

    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Init,
    Sync,
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Init {} => {
            // Create .gitspace and write the default template to it
            Config::new();
        }
        SubCommand::Sync {} => {
            //TODO: Read config path from CLI flag if it exists, otherwise use Paths::default()
            let paths = Paths::default();
            let config = Config::read_config_raw(Path::new(&paths.config));
            match &args.ssh_key {
                Some(_key) => config.clone_repos(Path::new(&args.ssh_key.unwrap())),
                None => config.clone_repos(Path::new(&config.ssh.identity_file)),            }
        }
    }
}
