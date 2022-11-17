//! Gitspace

// System dependencies
use std::fmt::{Display};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// CLI dependencies
use clap::{Parser, Subcommand};
mod config;
use config::{Config, PathType};

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
    /// Generate a .space directory with default config.json
    Init {},
    /// Clone/update repositories specified in config.json
    Sync {
        // /TODO: Allow users to put -s ~/.ssh/key_path at the end of the command by
        //migrating from CLI arg to Subcommand::Sync arg
    },
    /// Cleanup target path;
    Clean {
        #[clap(short, long)]
        target: String,
    },
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
            //TODO: Write integration test to ensure config_file override works properly
            let config_path = &args
                .config_file
                .unwrap_or_else(|| config.get_path_as_string(&PathType::Config));

            //TODO: Extract this into separate function so it can be used in other subcommands
            //without needing to rely on the default Config object
            let config = Config::read_config_raw(Path::new(&config_path));
            println!("{:#?}", &config);

            println!("🧱 Config path: {:?}", &config_path);
            //TODO: Write integration test to ensure ssh_key config.json override works properly
            let key_path = &args
                .ssh_key
                .unwrap_or_else(|| String::from(&config.ssh.identity_file));
                // .unwrap_or_else(|| String::from(&config.ssh.identity_file));

            println!("🧱 Key path: {:?}", key_path);
            let _ = &config.clone_repos(Path::new(&key_path));
        }
        SubCommand::Clean { target } => match target.as_str() {
            "space" | "s" => {
                let _ = &config.rm_space();
            }
            "config" | "c" => {
                let _ = &config.rm_config();
            }
            "repositories" | "r" => {
                let _ = &config.rm_repositories();
            }
            _ => {
                let _ = &config.rm_repositories();
            }
        },
    }
}
