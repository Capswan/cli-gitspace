//! Gitspace
// use std::io::prelude::*;
use clap::{Parser, Subcommand};
mod config;
use config::{Config, ConfigFile, ConfigParser, PathType, Paths};
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
    let config = Config::new();
    match args.cmd {
        SubCommand::Init {} => {
            // Create .gitspace and write the default template to it
            &config.write_config();
        }
        SubCommand::Sync {} => {
            //TODO: Read config path from CLI flag if it exists, otherwise use Paths::default()
            // let config_path = &config.get_path_as_string(PathType::Config);
            match &args.ssh_key {
                Some(_key) => config.clone_repos(Path::new(&args.ssh_key.unwrap())),
                None => config.clone_repos(Path::new(&config.ssh.identity_file)),
            }
        }
    }
}
