// use std::convert::From;
use dirs::home_dir;
use git2::{build, Cred, FetchOptions, RemoteCallbacks};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env::{current_dir, var};
use std::fs::{create_dir_all, read_dir, remove_dir_all, remove_file, write, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use symlink::{remove_symlink_dir, symlink_dir};

const GITSPACE: &str = ".space";
const CONFIG: &str = "config.json";
const REPOS: &str = "repositories";

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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    namespace: String,
    project: String,
    // symlink: String,
    // alias: String,
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
    pub repositories: Vec<Repo>,
    sync: Sync,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    pub space: String,
    pub config: String,
    pub repositories: String,
}

#[allow(dead_code)]
pub enum PathType {
    Space,
    Config,
    Repositories,
    Key,
}
pub fn cwd() -> String {
    if let Ok(current_dir) = current_dir() {
        if let Some(current_dir_str) = current_dir.to_str() {
            return current_dir_str.to_string();
        }
    }

    if let Ok(pwd) = var("PWD") {
        return pwd;
    }

    String::from('.')
}
impl Default for Paths {
    fn default() -> Self {
        Paths {
            space: String::from(GITSPACE),
            config: String::from(CONFIG),
            repositories: String::from(REPOS),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let key_path = home_dir()
            .unwrap()
            .join(".ssh/id_rsa")
            .to_str()
            .unwrap()
            .to_string();
        Config {
            paths: Paths {
                space: GITSPACE.to_string(),
                config: CONFIG.to_string(),
                repositories: REPOS.to_string(),
            },
            ssh: Ssh {
                host: "github".to_string(),
                host_name: "github.com".to_string(),
                user: "git".to_string(),
                identity_file: key_path,
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

// TODO: Create a getter that returns a tuple for config and repositories
impl Config {
    /// Create a new .space directory with empty repositories directory
    pub fn new() -> Self {
        let config = Self::default();
        let repositories_path = &config.get_path_as_string(&PathType::Repositories);
        create_dir_all(&repositories_path).unwrap();
        config
    }

    /// Write config.json inside .space directory
    pub fn write_config(&self) {
        let config_path = &self.get_path_as_string(&PathType::Config);
        // println!("{:?}", config_path);
        write(&config_path, &self.to_str()).unwrap();
    }

    /// Get common paths
    pub fn get_path_as_string(&self, path_type: &PathType) -> String {
        let space_path = String::from(&self.paths.space);
        // println!("{:?}", space_path);
        let config_path = format!("{}/{}", &self.paths.space, &self.paths.config);
        // println!("{:?}", config_path);
        let repositories_path = format!("{}/{}", &self.paths.space, &self.paths.repositories);
        // println!("{:?}", repositories_path);
        let key_path = &self.ssh.identity_file;
        // println!("{:?}", key_path);
        match path_type {
            PathType::Space => space_path,
            PathType::Config => config_path,
            PathType::Repositories => repositories_path,
            PathType::Key => key_path.to_owned(),
        }
    }

    /// Get a tuple of all common paths (space, config, repositories, key)
    pub fn get_paths_as_strings(&self) -> (String, String, String, String) {
        let space_path = &self.get_path_as_string(&PathType::Space);
        let config_path = &self.get_path_as_string(&PathType::Config);
        let repositories_path = &self.get_path_as_string(&PathType::Repositories);
        let ssh_key_path = &self.get_path_as_string(&PathType::Key);
        (
            space_path.to_string(),
            config_path.to_string(),
            repositories_path.to_string(),
            ssh_key_path.to_string(),
        )
    }

    fn exists(&self, path: &PathType) -> bool {
        let (space, config, repos, key) = &self.get_paths_as_strings();
        match path {
            PathType::Space => Path::new(space).exists(),
            PathType::Config => Path::new(config).exists(),
            PathType::Repositories => Path::new(repos).exists(),
            PathType::Key => Path::new(key).exists(),
        }
    }

    //TODO: Figure out why getting a no such file or directory when calling this from clone_repos
    fn dir_is_empty(&self, path_type: &PathType, repo_dir: &str) -> bool {
        println!("repo_dir {:?}", repo_dir);
        let path = match path_type {
            PathType::Space => self.get_path_as_string(&PathType::Space),
            PathType::Config => self.get_path_as_string(&PathType::Config),
            PathType::Repositories => self.get_path_as_string(&PathType::Repositories),
            PathType::Key => self.get_path_as_string(&PathType::Key),
        };
        let repo_path = format!("{}/{}", path, repo_dir);
        let repo_path = Path::new(&repo_path);
        // println!("{:?}", repo_path);
        if repo_path.exists() {
            let mut entries = match repo_path.read_dir() {
                Ok(entries) => entries,
                Err(_) => return false,
            };
            entries.next().is_none()
        } else {
            false
        }
    }

    /// Return a JSON value of the config.json file
    // fn read_config_json(config_path: &Path) -> Value {
    //     // println!("{:?}", config_path);
    //     let file = File::open(&config_path).unwrap();
    //     let reader = BufReader::new(file);
    //     let value: Value = serde_json::from_reader(reader).unwrap();
    //     value
    // }

    /// Return a Config struct of the .gitspace file
    pub fn read_config_raw(config_path: &Path) -> Config {
        // println!("{:?}", config_path);
        let file = File::open(&config_path).unwrap();
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).unwrap();
        config
    }

    /// remove the .gitspace/config.json file
    pub fn rm_config(&self) {
        let config_path = &self.get_path_as_string(&PathType::Config);
        remove_file(&config_path).unwrap();
        println!("???? Removed config.json");
    }

    /// remove the .gitspace/repositories directory
    pub fn rm_repositories(&self) {
        let repositories_path = &self.get_path_as_string(&PathType::Repositories);
        remove_dir_all(&repositories_path).unwrap();
        println!("???? Removed repositories directory");
    }

    /// create symlinks in cwd based on newly cloned repositories in ~/.space/repositories
    /// return a vector of symlinks created
    pub fn write_symlinks(repositories: &Vec<Repo>) -> Vec<(String, String)> {
        //TODO: Add a check to see if the symlink already exists
        let mut symlinks: Vec<(String, String)> = Vec::new();
        repositories.iter().for_each(|repo| {
            let (space_path, _, repos_path, _) = Config::default().get_paths_as_strings();
            let project_src_path = format!("{}/{}", &repos_path, &repo.project);
            let project_dest_path = format!("{}/{}", String::from(cwd()), &repo.project);

            // println!("???? space_path: {}", &space_path);
            // println!("???? project_src_path: {}", &project_src_path);
            // println!("???? project_dest_path: {:#?}", &project_dest_path);
            // println!("???? repos_path: {}", &repos_path);

            symlink_dir(&project_src_path, &project_dest_path).unwrap();
            let src_and_dest_paths = (project_src_path, project_dest_path);
            // println!("???? src_and_dest_paths: {:?}", &src_and_dest_paths);
            symlinks.push(src_and_dest_paths);
        });
        symlinks
    }

    pub fn rm_symlinks(&self) {
        // let mut removed_symlinks: Vec<String> = Vec::new();
        let entries = match Path::new(&cwd()).read_dir() {
            Ok(entries) => entries,
            Err(_) => return,
        };
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_symlink() {
                //TODO: Only remove symlinks if they match the project name in the config.json file
                println!("???? Removing symlink: {:?}", path);
               remove_file(&path).unwrap(); 
            }
        }
    }

    /// remove the .gitspace directory
    pub fn rm_space(&self) {
        let space_path = &self.get_path_as_string(&PathType::Space);
        remove_dir_all(&space_path).unwrap();
        println!("???? Removed .space directory");
    }

    /// Clone all repositories from config.json
    pub fn clone_repos(&self, key_path: &Path) {
        self.repositories.iter().for_each(|repo| {
            // Instantiate buider for cloning, callbacks for processing credentials/auth request
            println!("???? repo.project: {:?}", &repo.project);
            if Path::new(&self.get_path_as_string(&PathType::Repositories)).exists() {
                println!("???? repo exists");
                if !self.dir_is_empty(&PathType::Repositories, &repo.project) {
                    let mut callbacks = RemoteCallbacks::new();
                    callbacks.credentials(|_url, username_from_url, _allowed_types| {
                        Cred::ssh_key(username_from_url.unwrap(), None, key_path, None)
                    });
                    let mut fetch_options = FetchOptions::new();
                    let _ = &fetch_options.remote_callbacks(callbacks);

                    // println!("repo.project: {}", &repo.project);
                    let repo_uri = format!(
                        "git@{}:{}/{}",
                        &self.ssh.host_name, &repo.namespace, &repo.project
                    );

                    println!("???? dir is empty");
                    let mut repo_dir = PathBuf::new();
                    repo_dir.push(&self.get_path_as_string(&PathType::Repositories));
                    repo_dir.push(&repo.project);

                    println!("???? Cloning {} into {}", &repo.project, &repo_dir.display());
                    create_dir_all(&repo_dir).unwrap();
                    println!(
                        "???? Cloning {} into {}",
                        &repo_uri,
                        &repo_dir.display().to_string()
                    );
                    let mut builder = build::RepoBuilder::new();
                    builder.fetch_options(fetch_options);
                    builder.clone(&repo_uri, &repo_dir).unwrap();
                }
                // else {
                //     println!("???? {} already exists", &repo.project);
                //     println!("Pulling {}", &repo_uri);
                //     let repo = Repository::open(&repo_uri).unwrap();
                //     let mut remote = repo.find_remote("origin").unwrap();
                //     //TODO: Handle multiple branch names, not just master
                //     remote
                //         .fetch(&["master"], Some(&mut fetch_options), None)
                //         .unwrap();
                // }
            } else {
                //TODO: Add the fetch case
                println!("???? repositories directory does not exist. Please init first");
            }
        });
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
    #[ignore]
    fn gitspace_is_generated() {
        // This test works but it's ignored because otherwise customized config.json changes will be overwritten on every consecutive test run
        if Path::new(".gitspace").exists() {
            remove_dir_all(".gitspace").unwrap();
        }
        let config = Config::new();
        let _ = &config.write_config();
        // exists checks the path stored on Config; ie. ".gitspace"
        let config_exists = config.exists(&PathType::Config);
        assert!(config_exists);

        // Comment this out to toggle the removal of the .space directory
        // config.rm_space();
        // assert_eq!(config.exists(&PathType::Space), false);
    }

    /// Creating a raw Config, converting it to a JSON Value, and then comparing it to Default Config type
    #[test]
    fn config_to_json() {
        // WARN: This could fail on other systems (eg. Windows) since default will change the home dir
        // If this test fails, can simply ignore
        let config_default = Config::default();

        let key_path = &config_default.get_path_as_string(&PathType::Key);
        let config_raw = Config {
            paths: Paths {
                space: String::from(GITSPACE),
                config: String::from(CONFIG),
                repositories: String::from(REPOS),
            },
            ssh: Ssh {
                host: "github".to_string(),
                host_name: "github.com".to_string(),
                user: "git".to_string(),
                identity_file: String::from(key_path),
            },
            repositories: vec![
                Repo {
                    namespace: "capswan".to_string(),
                    project: "cli-gitspace".to_string(),
                    // alias: "gsp".to_string(),
                    // symlink: "cli-gitspace".to_string(),
                },
                Repo {
                    namespace: "capswan".to_string(),
                    project: "cli-ftr".to_string(),
                    // alias: "ftr".to_string(),
                    // symlink: "cli-ftr".to_string(),
                },
            ],
            sync: Sync {
                enabled: true,
                cron: "30 0 * * *".to_string(),
            },
        };

        let config_default_json = config_default.to_json();
        assert_eq!(config_default_json, config_raw.to_json());
    }
    #[test]
    fn get_space_path() {
        let config = Config::default();
        let space_path = &config.get_path_as_string(&PathType::Space);
        assert_eq!(space_path.as_str(), GITSPACE);
    }
    #[test]
    fn get_config_path() {
        let config = Config::default();
        let config_path = &config.get_path_as_string(&PathType::Config);
        assert_eq!(config_path.as_str(), format!("{}/{}", GITSPACE, CONFIG));
    }
    #[test]
    fn get_repositories_path() {
        let config = Config::default();
        let repositories_path = &config.get_path_as_string(&PathType::Repositories);
        assert_eq!(
            repositories_path.as_str(),
            format!("{}/{}", GITSPACE, REPOS)
        );
    }
}
