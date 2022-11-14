mod config;
use config::{Config, Repository};
use git2::Repository;
// https://docs.rs/git2/0.15.0/git2/

// Config {
//     path: PathBuf::from(".gitspace"),
//     ssh: Ssh {
//         host: "github".to_string(),
//         host_name: "github.com".to_string(),
//         user: "git".to_string(),
//         identity_file: "~/.ssh/id_rsa".to_string(),
//     },
//     repositories: vec![Repository {
//         namespace: "capswan".to_string(),
//         project: "cli-gitspace".to_string(),
//     }],
//     sync: Sync {
//         enabled: true,
//         cron: "30 0 * * *".to_string(),
//     },
// }

pub trait ConfigParser {
    fn get_repos(&self) -> Vector<config::Repository>;
}

impl ConfigParser for Config {
    fn get_repos(&self) -> Vector<config::Repository> {
        let repos = &self.repositories.into_iter().map(|repo| {
            let url = format!(
                "git@{}:{}/{}.git",
                &self.ssh.host_name, repo.namespace, repo.project
            );
            let path = format!("{}/{}", ".repos", repo.project);
            let repo = Repository::clone(url, path);
            repo
        });
       repos 
    }
}

mod tests {
    use super::*;

    #[test]
    fn get_repos() {
        let config = Config::new();
        let repos = config.get_repos();
        println!("{:?}", repos);
        assert_eq!(repos, config.repositories);
    }
}
