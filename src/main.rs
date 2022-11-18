//! Gitspace
use std::fmt::{Display};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
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
    /// Cleanup target path; defaults to cleaning up repositories directory
    Clean {
        #[clap(short, long)]
        target: String,
    },
    Symlink {}
}

fn main() {
    let args = Arguments::parse();
    //TODO: Remove config.json and .space path as options from config.json (Paths struct); easier to just assume
    //those paths; if user changes the config.json path has changed how would we read it anyways
    //unless they specify the --config_path everytime? And if they did, would we implement caching
    //to update the config with the new path? Could make sense, but for now targeting the golden
    //path case of "I run gitspace init && gitspace sync and it just works"
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
        SubCommand::Symlink {} => {
            //TODO: Allow users to specify a target symlink directory, default to CWD as root.
            //Update Paths struct to include symlink path

            Config::write_symlinks(&config.repositories);
        }
        SubCommand::Clean { target } => match target.as_str() {
            "space" | "s" => {
                let _ = &config.rm_space();
            }
            "config" | "c" => {
                let _ = &config.rm_config();
            }
            "repositories" | "r" => {
                //TODO: Check nested directories for whether git directories have any uncommitted changes before removing
                let _ = &config.rm_repositories();
            }
            "symlinks" | "l" => {
                //TODO: Allow user to specify location of symlinks with a new flag on the Clean
                //subcommand
                let _ = &config.rm_symlinks(None);
            }
            _ => {
                let _ = &config.rm_repositories();
            }
        },
    }
}
