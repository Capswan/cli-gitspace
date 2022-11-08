use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    ssh: Ssh,
    repositories: Vec<Repository>,
    sync: Sync,
}

impl Default for Config {
    fn default() -> Self {
        Config {
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

pub trait ConfigTemplate {
    // TODO: Consider replacing below with From & Into
    // https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
    fn to_config(self) -> Config;
    fn to_json(self) -> Value;
    fn to_str(self) -> String;
}

impl ConfigTemplate for Value {
    fn to_config(self) -> Config {
        serde_json::from_value(self).unwrap()
    }
    fn to_json(self) -> Self {
        self
    }
    fn to_str(self) -> String {
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
    fn to_str(self) -> String {
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
    fn to_str(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creating a raw Config, converting it to a JSON Value, and then comparing it to Default Config type
    #[test]
    fn config_to_json() {
        let config_raw = Config {
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
    #[test]
    fn string_to_config() {
        let config_str = r#"{ "ssh": { "host": "github", "hostName": "github.com", "user": "git", "identityFile": "~/.ssh/id_rsa" }, "repositories": [ { "namespace": "capswan", "project": "cli-gitspace" } ], "sync": { "enabled": true, "cron": "30 0 * * *" } }"#.to_owned();
        let config_default = Config::default();
        assert_eq!(config_default, config_str.to_config());
    }
}

//     #[test]
//     fn serialized_json_matches_deserialized_config() {
//         let config = Config {
//             ssh: Ssh {
//                 host: "github".to_string(),
//                 host_name: "github.com".to_string(),
//                 user: "git".to_string(),
//                 identity_file: "~/.ssh/id_rsa".to_string(),
//             },
//             repositories: vec![Repository {
//                 namespace: "cli-gitspace".to_string(),
//                 project: "cli".to_string(),
//             }],
//             sync: Sync {
//                 enabled: true,
//                 cron: "30 0 * * *".to_string(),
//             },
//         };

//         // Create file with template serialized from the Config struct above
//         // let serialized_config = serde_json::to_string(&config).unwrap();
//         // let serialized_json = serde_json::to_string(&TEMPLATE).unwrap();

//         // assert_eq!(serialized_config, serialized_json);
//         unimplemented!()
//     }
// }
// let mut TEMPLATE: serde_json::Value = json!({
//     "ssh": {
//         "host": "github",
//         "hostName": "github.com",
//         "user": "git",
//         "identityFile": "~/.ssh/id_rsa"
//     },
//     "repositories": [
//         {
//             "namespace": "capswan",
//             "project": "cli-gitspace"
//         }
//     ],
//     "sync": {
//         "enabled": true,
//         "cron": "0 0 * * *"
//     }
// });
