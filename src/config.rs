use git2::Repository;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs::{create_dir_all, remove_dir_all, remove_file, write, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

const GITSPACE: &str = ".gitspace";
/// Refers to your ~/.ssh/config file
/// Struct parameters example:
/// Host github
///    HostName github.com
///    User git
///    IdentityFile ~/.ssh/id_rsa
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ssh {
    host: String,
    host_name: String,
    user: String,
    identity_file: String,
}

/// Single repository within an array of repositories inside the .gitspace file
/// Struct parameters example:
/// - github.com/capswan/cli-gitspace
/// - github.com/{namespace}/{repository}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    namespace: String,
    project: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sync {
    enabled: bool,
    cron: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    paths: Paths,
    ssh: Ssh,
    repositories: Vec<Repo>,
    sync: Sync,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    space: PathBuf,
    config: PathBuf,
    repositories: PathBuf,
}

enum PathType {
    Space,
    Config,
    Repositories,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            paths: Paths {
                space: PathBuf::from(GITSPACE),
                config: PathBuf::from(".gitspace/config.json"),
                repositories: PathBuf::from(".gitspace/repositories"),
            },
            ssh: Ssh {
                host: "github".to_string(),
                host_name: "github.com".to_string(),
                user: "git".to_string(),
                identity_file: "~/.ssh/id_rsa".to_string(),
            },
            repositories: vec![Repo {
                namespace: "capswan".to_string(),
                project: "cli-gitspace".to_string(),
            }],
            sync: Sync {
                enabled: true,
                cron: "30 0 * * *".to_string(),
            },
        }
    }
}

pub trait ConfigFile {
    fn new() -> Self;
    fn exists(&self, path: PathType) -> bool;
    fn read_config(&self) -> Value;
    fn rm_config(&self);
    fn rm_repositories(&self);
    fn rm_space(&self);
}

impl ConfigFile for Config {
    /// Create a new .gitspace file in the current working directory
    fn new() -> Self {
        let config = Self::default();
        create_dir_all(&config.paths.repositories).unwrap();
        write(&config.paths.config, &config.to_str()).unwrap();
        config
    }

    /// Checks if the .gitspace file exists
    fn exists(&self, path: PathType) -> bool {
        match path {
            PathType::Space => Path::new(&self.paths.space).exists(),
            PathType::Config => Path::new(&self.paths.config).exists(),
            PathType::Repositories => Path::new(&self.paths.repositories).exists(),
            _ => false,
        }
    }

    /// Return a JSON value of the .gitspace file
    fn read_config(&self) -> Value {
        let file = File::open(&self.paths.config).unwrap();
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader).unwrap();
        value
    }

    /// remove the .gitspace/config.json file
    fn rm_config(&self) {
        remove_file(&self.paths.config).unwrap();
    }

    /// remove the .gitspace/repositories directory
    fn rm_repositories(&self) {
        // remove_file(&self.paths.repositories).unwrap();
        remove_dir_all(&self.paths.repositories).unwrap();
    }

    /// remove the .gitspace directory
    fn rm_space(&self) {
        remove_dir_all(&self.paths.space).unwrap();
    }
}
pub trait ConfigTemplate {
    //TODO: Consider replacing to_config & to_json with From & Into
    //TODO: Consider replacing to_str with Display trait
    // https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
    fn to_config(self) -> Config;
    fn to_json(self) -> Value;
    fn to_str(&self) -> String;
}

impl ConfigTemplate for Value {
    fn to_config(self) -> Config {
        serde_json::from_value(self).unwrap()
    }
    fn to_json(self) -> Self {
        self
    }
    fn to_str(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl ConfigTemplate for Config {
    fn to_config(self) -> Self {
        self
    }
    fn to_json(self) -> Value {
        serde_json::to_value(self).unwrap()
    }
    fn to_str(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl ConfigTemplate for String {
    fn to_config(self) -> Config {
        serde_json::from_str(self.as_str()).unwrap()
    }
    fn to_json(self) -> Value {
        serde_json::from_str(self.as_str()).unwrap()
    }
    fn to_str(&self) -> String {
        String::from(self)
    }
}

trait ConfigParser {
    fn clone_repos(&self);
}

impl ConfigParser for Config {
    fn clone_repos(&self) {
        self.clone().repositories.into_iter().map(|repo| {
            let url = format!(
                "git@{}:{}/{}.git",
                &self.ssh.host_name, repo.namespace, repo.project
            );
            let path = format!("{}/{}/{}", ".gitspace", ".repos", repo.project);
            Repository::clone(&url, path).unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gitspace_is_generated() {
        let template: serde_json::Value = json!({
            "paths": {
                "space": ".gitspace",
                "config": ".gitspace/config.json",
                "repositories": ".gitspace/repositories"
            },
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
        let config_exists = Config::exists(&config, PathType::Space);
        assert!(config_exists);
    }

    #[test]
    fn can_read_config() {
        let config = Config::new();
        let config_file = Config::read_config(&config);
        assert_eq!(config_file, serde_json::to_value(&config).unwrap());
    }
    /// Creating a raw Config, converting it to a JSON Value, and then comparing it to Default Config type
    #[test]
    fn config_to_json() {
        let config_raw = Config {
            paths: Paths {
                space: PathBuf::from(".gitspace"),
                config: PathBuf::from(".gitspace/config.json"),
                repositories: PathBuf::from(".gitspace/repositories"),
            },
            ssh: Ssh {
                host: "github".to_string(),
                host_name: "github.com".to_string(),
                user: "git".to_string(),
                identity_file: "~/.ssh/id_rsa".to_string(),
            },
            repositories: vec![Repo {
                namespace: "capswan".to_string(),
                project: "cli-gitspace".to_string(),
            }],
            sync: Sync {
                enabled: true,
                cron: "30 0 * * *".to_string(),
            },
        };

        let config_default_json = Config::default().to_json();
        assert_eq!(config_default_json, config_raw.to_json());
    }

    #[test]
    fn clone_repos() {
        let config = Config::new();
        let repos = config.clone_repos();
        println!("{:?}", repos);
        // assert_eq!(repos, config.repositories);
    }

    #[test]
    fn cleanup() {
        let config = Config::new();
        config.rm_config();
        assert_eq!(config.exists(PathType::Config), false);
        config.rm_repositories();
        assert_eq!(config.exists(PathType::Repositories), false);
        config.rm_space();
        assert_eq!(config.exists(PathType::Space), false);
    }
}
