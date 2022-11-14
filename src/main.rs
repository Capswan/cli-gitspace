//! Gitspace
use clap::{Parser, Subcommand};
// use serde;
use serde_json::json;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

mod config;
use config::{ConfigFile, ConfigTemplate};

const GITSPACE: &str = ".gitspace";

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

fn delete_file(path: &str) {
    fs::remove_file(path).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, ConfigFile, ConfigTemplate};

    #[test]
    fn gitspace_is_generated() {
        let template: serde_json::Value = json!({
            "path": ".gitspace",
            "ssh": {
                "host": "github",
                "hostName": "github.com",
                "user": "git",
                "identityFile": "~/.ssh/id_rsa"
            },
            "repositories": [
                {
                    "namespace": "capswan",
                    "project": "cli-gitspace"
                }
            ],
            "sync": {
                "enabled": true,
                "cron": "30 0 * * *"
            }
        });
        // Convert template from JSON to a Config
        let deserialized = serde_json::to_value(&template).unwrap();
        let config = deserialized.to_config();
        // exists checks the path stored on Config; ie. ".gitspace"
        assert_eq!(Config::exists(&config), true);
    }
}
