use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{write, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
// use std::convert::From;

/// Refers to your ~/.ssh/config file
/// Struct parameters example:
/// Host github
///    HostName github.com
///    User git
///    IdentityFile ~/.ssh/id_rsa
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    namespace: String,
    project: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sync {
    enabled: bool,
    cron: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    path: PathBuf,
    ssh: Ssh,
    repositories: Vec<Repository>,
    sync: Sync,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: PathBuf::from(".gitspace"),
            ssh: Ssh {
                host: "github".to_string(),
                host_name: "github.com".to_string(),
                user: "git".to_string(),
                identity_file: "~/.ssh/id_rsa".to_string(),
            },
            repositories: vec![Repository {
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
    //TODO: Remove wonky workaround with &self (for .gitspace path)
    fn new() -> Self;
    fn exists(&self) -> bool;
    fn read_config(&self) -> Value;
}

impl ConfigFile for Config {
    /// Create a new .gitspace file in the current working directory
    fn new() -> Self {
        let config = Self::default();
        write(&config.path, &config.to_str());
        config
    }

    /// Checks if the .gitspace file exists
    fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    /// Return a JSON value of the .gitspace file
    fn read_config(&self) -> Value {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader).unwrap();
        value
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

#[cfg(test)]
mod tests {
    use super::*;

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
            path: PathBuf::from(".gitspace"),
            ssh: Ssh {
                host: "github".to_string(),
                host_name: "github.com".to_string(),
                user: "git".to_string(),
                identity_file: "~/.ssh/id_rsa".to_string(),
            },
            repositories: vec![Repository {
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
    // #[test]
    // fn string_to_config() {
    //     let config_str = r#"{ "ssh": { "host": "github", "hostName": "github.com", "user": "git", "identityFile": "~/.ssh/id_rsa" }, "repositories": [ { "namespace": "capswan", "project": "cli-gitspace" } ], "sync": { "enabled": true, "cron": "30 0 * * *" } }"#.to_owned();
    //     let config_default = Config::default();
    //     assert_eq!(config_default, config_str.to_config());
    // }
}
