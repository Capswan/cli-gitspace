//! Gitspace

// System dependencies
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// CLI dependencies
use clap::{Parser, Subcommand};
mod config;
use config::{get_key_path, Config, PathType};

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

// Similar to PathType, but not keeping separate to ensure Key isn't used (don't want to enable users to remove their own SSH key for obvious reasons)
enum CleanType {
    Space,
    Config,
    Repositories,
}

impl Display for CleanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanType::Space => write!(f, "space"),
            CleanType::Config => write!(f, "config"),
            CleanType::Repositories => write!(f, "repositories"),
        }
    }
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Generate a .space directory with default config.json
    Init {},
    /// Clone/update repositories specified in config.json
    Sync {},
    /// Cleanup target path;
    Clean {
        #[clap(short, long, default_value_t = String::from("space"))]
        target: CleanType,
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
            let config_path = &args
                .config_file
                .unwrap_or_else(|| config.get_path_as_string(PathType::Config));

            //TODO: Extract this into separate function so it can be used in other subcommands
            //without needing to rely on the default Config object
            let config = Config::read_config_raw(Path::new(&config_path));
            println!("{:#?}", &config);

            println!("🧱 Config path: {:?}", &config_path);
            let key_path = &args
                .ssh_key
                .unwrap_or_else(|| String::from(&config.ssh.identity_file));

            println!("🧱 Key path: {:?}", key_path);
            config.clone_repos(Path::new(&key_path));
        }
        SubCommand::Clean { target } => match target {
            CleanType::Space => {
                println!("🧱 Removing .space directory");
                let _ = &config.rm_space();
            }
            CleanType::Config => {
                println!("🧱 Removing config.json");
                let _ = &config.rm_config();
            }
            CleanType::Repositories => {
                println!("🧱 Removing repositories directory");
                let _ = &config.rm_repositories();
            }
            _ => {
                println!("🧱 Cleaning repositories");
                let _ = &config.rm_repositories();
            }
        },
    }
}
