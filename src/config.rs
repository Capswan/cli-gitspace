use git2::{build, Cred, FetchOptions, RemoteCallbacks, Repository};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub identity_file: String,
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
    pub ssh: Ssh,
    repositories: Vec<Repo>,
    sync: Sync,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    pub space: PathBuf,
    pub config: PathBuf,
    pub repositories: PathBuf,
}

#[allow(dead_code)]
pub enum PathType {
    Space,
    Config,
    Repositories,
}

impl Default for Paths {
    fn default() -> Self {
        Paths {
            space: PathBuf::from(".gitspace"),
            config: PathBuf::from(".gitspace/config.json"),
            repositories: PathBuf::from(".gitspace/repositories"),
        }
    }
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
            repositories: vec![
                Repo {
                    namespace: "capswan".to_string(),
                    project: "cli-gitspace".to_string(),
                },
                Repo {
                    namespace: "capswan".to_string(),
                    project: "cli-ftr".to_string(),
                },
            ],
            sync: Sync {
                enabled: true,
                cron: "30 0 * * *".to_string(),
            },
        }
    }
}

//TODO: Move methods into Config impl; no need for trait
pub trait ConfigFile {
    fn exists(&self, path: PathType) -> bool;
    fn read_config_raw(config_path: &Path) -> Config;
    fn read_config(config_path: &Path) -> Value;
    fn rm_config(&self);
    fn rm_repositories(&self);
    fn rm_space(&self);
}

impl Config {
    /// Create a new .gitspace file in the current working directory
    pub fn new() -> Self {
        let config = Self::default();
        create_dir_all(&config.paths.repositories).unwrap();
        write(&config.paths.config, &config.to_str()).unwrap();
        config
    }
}

impl ConfigFile for Config {
    /// Checks if the .gitspace file exists
    fn exists(&self, path: PathType) -> bool {
        match path {
            PathType::Space => Path::new(&self.paths.space).exists(),
            PathType::Config => Path::new(&self.paths.config).exists(),
            PathType::Repositories => Path::new(&self.paths.repositories).exists(),
        }
    }

    /// Return a JSON value of the .gitspace file
    fn read_config(config_path: &Path) -> Value {
        let file = File::open(&config_path).unwrap();
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader).unwrap();
        value
    }

    /// Return a Config struct of the .gitspace file
    fn read_config_raw(config_path: &Path) -> Config {
        let file = File::open(&config_path).unwrap();
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).unwrap();
        config
    }
    /// remove the .gitspace/config.json file
    fn rm_config(&self) {
        println!("Removing {}", &self.paths.config.display());
        remove_file(&self.paths.config).unwrap();
    }

    /// remove the .gitspace/repositories directory
    fn rm_repositories(&self) {
        println!("Removing {}", &self.paths.repositories.display());
        remove_dir_all(&self.paths.repositories).unwrap();
    }

    /// remove the .gitspace directory
    fn rm_space(&self) {
        println!("Removing {}", &self.paths.space.display());
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

pub trait ConfigParser {
    fn clone_repos(&self, key_path: &Path);
}

impl ConfigParser for Config {
    fn clone_repos(&self, key_path: &Path) {
        //TODO: Iterate through every repository
        //TODO: Clone each repository if it doesn't exist
        //TODO: Pull each repository if it does exist
        //TODO: Print a list of repositories and their status

        self.repositories.iter().for_each(|repo| {
            // Instantiate buider for cloning, callbacks for processing credentials/auth request
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                Cred::ssh_key(username_from_url.unwrap(), None, &key_path, None)
            });
            let mut fetch_options = FetchOptions::new();
            &fetch_options.remote_callbacks(callbacks);

            let repo_path = format!(
                "git@{}:{}/{}",
                &self.ssh.host_name, &repo.namespace, &repo.project
            );

            if !Path::new(&repo.project).exists() {
                //TODO: Allow users to put -s ~/.ssh/key_path at the end of the command
                //TODO: move occurs because fetch_options has type `git2::FetchOptions`, which does not implement the `Copy` trait

                println!("Cloning {}", &repo_path);
                create_dir_all(&repo.project).unwrap();
                let mut builder = build::RepoBuilder::new();
                builder.fetch_options(fetch_options);
                //TODO: Append the project to the path; replace self.paths.repositories with
                //PathBuf
                let mut repo_directory = PathBuf::new();
                repo_directory.push(&self.paths.repositories);
                repo_directory.push(&repo.project);

                // let repo_directory: PathBuf = [&self.paths.repositories, &repo.project].iter().collect();
                builder.clone(&repo_path, &repo_directory).unwrap();
            }
            // else {
            //     println!("Pulling {}", &repo_path);
            //     let repo = Repository::open(&repo_path).unwrap();
            //     let mut remote = repo.find_remote("origin").unwrap();
            //     //TODO: Handle multiple branch names, not just master
            //     remote
            //         .fetch(&["master"], Some(&mut fetch_options), None)
            //         .unwrap();
            // }
        });
    }
}
// self.clone().repositories.into_iter().map(|repo| {
//     let url = format!(
//         "git@{}:{}/{}.git",
//         &self.ssh.host_name, repo.namespace, repo.project
//     );
//     let path = format!("{}/{}/{}", ".gitspace", ".repos", repo.project);
//     // println!("")
//     Repository::clone(&url, path).unwrap();
// }).collect::<Vec<_>>();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gitspace_is_generated() {
        let template: Value = serde_json::json!({
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
        let config_exists = config.exists(PathType::Space);
        assert!(config_exists);

        // Comment this out to toggle the removal of the .gitspace directory
        // config.rm_space();
        // assert_eq!(config.exists(PathType::Space), false);
    }

    #[test]
    fn can_read_config() {
        let config = Config::new();
        let config_file = Config::read_config(&config.paths.config);
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

    // #[test]
    // fn clone_repos() {
    //     let config = Config::new();
    //     let repos = config.clone_repos();
    //     println!("{:?}", repos);
    //     // assert_eq!(repos, config.repositories);
    // }
}
