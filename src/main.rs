//! Gitspace
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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

const TEMPLATE: &str = r#"
    {
        "ssh": {
            "host": "github",
            "hostName": "github.com",
            "user": "git",
            "identityFile": "~/.ssh/id_rsa",
        },
        "repositories": [
            { "capswan": "cli" }
        ],
        "sync": {
            "enabled": "true"
            "cron": "30 0 * * *",
        }
    }
"#;

/// Refers to your ~/.ssh/config file
/// Struct parameters example:
/// Host github
///    HostName github.com
///    User git
///    IdentityFile ~/.ssh/id_rsa
struct SSH {
    host: String,
    host_name: String,
    user: String,
    identity_file: String,
}

/// Single repository within an array of repositories inside the .gitspace file
/// Struct parameters example:
/// - github.com/capswan/cli-gitspace
/// - github.com/{namespace}/{repository}
struct Repository {
    namespace: String,
    project: String,
}

struct Sync {
    enabled: bool,
    cron: String,
}

const GITSPACE: &str = ".gitspace";

fn path_exists(path: &str) -> bool {
    Path::new(path).try_exists().unwrap()
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Init {} => {
            println!("init");
            // if the config file already exists, do nothing
            if path_exists(GITSPACE) {
                println!("gitspace already exists");
            } else {
                // create a .gitspace file in the current working directory
                let mut file = File::create(GITSPACE).unwrap();
                // write the default config to it
                file.write_all(TEMPLATE.as_bytes()).unwrap();
            }
        }
        SubCommand::Sync {} => {
            println!("sync");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_can_parse_gitspace() {
        assert_eq!(path_exists("src/main.rs"), true);
        assert_eq!(path_exists("src/main.rsx"), false);
    }
}
