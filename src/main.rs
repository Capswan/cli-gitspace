//! Gitspace
use clap::{Parser, Subcommand};
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use serde;
use serde_json::{json};

mod config;
use config::ConfigTemplate;

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

fn path_exists(path: &str) -> bool {
    Path::new(path).try_exists().unwrap()
}

fn create_file(path: &str, content: &str) -> fs::File {
    // create a .gitspace file in the current working directory
    let mut template = config::Config::default();

    let mut file = fs::File::create(path).unwrap();
    // write the default config to it
    file.write_all(content.as_bytes()).unwrap();
    file
}

fn delete_file(path: &str) {
    fs::remove_file(path).unwrap();
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Init {} => {
            println!("init");
            // if the config file already exists, do nothing
            if path_exists(GITSPACE) {
                println!("gitspace already exists");
                //TODO: Prompt the user to create new .gitspace
                //TODO: Move previous .gitspace to .gitspace.bak
                //TODO: Prompt user to select from gitspace template or interactive
            } else {
                // Grab default template
                let template = config::Config::default();
                
                // Convert default template
                let template_str = template.to_str();

                // Create .gitspace and write the default template to it
                create_file(GITSPACE, template_str.as_str());
            }
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

    #[test]
    fn gitspace_is_generated() {
        let mut template: serde_json::Value = json!({
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

        create_file(GITSPACE, serde_json::to_string(&template).unwrap().as_str());
        assert_eq!(path_exists(GITSPACE), true);
    }
}
