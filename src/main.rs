//! Gitspace
use clap::{Parser, Subcommand};
use std::io::prelude::*;
use std::path::Path;

mod config;
use config::{ConfigFile, ConfigTemplate};

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
    Init,
    Sync,
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Init {} => {
            // Grab default template
            // let template = config::Config::default();

            // Create .gitspace and write the default template to it
            config::Config::new();
        }
        SubCommand::Sync {} => {
            //TODO: Iterate through every repository
            //TODO: Clone each repository if it doesn't exist
            //TODO: Pull each repository if it does exist
            //TODO: Print a list of repositories and their status
            todo!();
        }
    }
}
