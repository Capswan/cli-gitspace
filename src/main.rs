//! Gitspace

// System dependencies
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// CLI dependencies
use clap::{Parser, Subcommand};
mod config;
use config::{Config, PathType, get_key_path};

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
    match &args.cmd {
        SubCommand::Init {} => {
            // Create .gitspace and write the default template to it
            let _ = &config.write_config();
        }
        SubCommand::Sync {} => {
            //TODO: Read config path from CLI flag if it exists, otherwise use Paths::default()
            // let config_path = &config.get_path_as_string(PathType::Config);
            let config_path = &args
                .config_file
                .unwrap_or_else(|| config.get_path_as_string(PathType::Config));

            let config = Config::read_config_raw(Path::new(&config_path));
            println!("{:#?}", &config);

            println!("🧱 Config path: {:?}", &config_path);
            let key_path = &args.ssh_key.unwrap_or_else(|| {
                String::from(&config.ssh.identity_file)
            });
            println!("🧱 Key path: {:?}", key_path);
            // let mut file = File::open(Path::new(key_path)).expect("File not found");
            // let mut data = String::new();
            // file.read_to_string(&mut data).expect("Error reading file");
            // println!("🧱 Key content: {:?}", data);
            config.clone_repos(Path::new(&key_path));
        }
    }
}
