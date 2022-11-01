# gitspace

Git-based workspaces

- Similar to submodules without the check-in
- Language agnostic
- SSH config pass-through for authorization

---

## Getting Started
1. `cargo install gitspace`
2. `gitspace init`
3. Update config
    - Add your ssh host (info below)
    - Add your repos
4. `gitspace sync`

### Commands

#### Commands::`Initialize`

| Name | Description                                    |
| :--- | :--------------------------------------------- |
| init | Create a new gitspace config                    |
| sync | Clone repos, update symlinks, update gitignore |

#### Commands::`Maintain`

| Name             | Description                                                   |
| :--------------- | :------------------------------------------------------------ |
| alias            | generate aliases for gitspace repos, defaults to `alias --zsh` |
| alias --bash     | generate bash / zsh compatible aliases                        |
| alias --nushell  | generate nushell aliases                                      |
| ignore           | Update (or create) new gitignore file based on cloned repos   |
| clean            | Without argument, defaults to `clean --all`                   |
| clean --all      | removes everything besides gitignore and your gitspace config  |
| clean --symlinks | Remove all gitspace generated symlinks                         |
| clean --repos    | Remove all cloned repos (ie. `.repos` directory)              |
| fetch            | Fetch all updates from master for local repos                 |
| version          | print gitspace version                                         |

---

## Space

- A space...
  - is short for "workspace"
  - is any directory with a .gitspace file
  - combines multiple repositories into one place

## Git

- Git is how you configure...
  - Which repositories should be stored
    - How they should be nested
  - Your auth strategy
  - Your sync policy
    - How often it should run

---

## Dependencies

| Name            | Link                                                                   | Description                                                  |
| :-------------- | :--------------------------------------------------------------------- | :----------------------------------------------------------- |
| git2            | [git2](https://docs.rs/git2/latest/git2/build/struct.RepoBuilder.html) | Clone git repos                                              |
| serde           | [serde](https://lib.rs/crates/serde)                                   | Serializing/deserializing (for payloads and repogen config ) |
| symlink         | [symlink](https://docs.rs/symlink/0.1.0/symlink/)                      | Cross-platform symlinks                                      |
| reqwest-graphql | [reqwest-graphql](https://crates.io/crates/reqwest-graphql/1.0.0)      | Querying Github's GraphQL API                                |
| clap            | [clap](https://crates.io/crates/clap)                                  | CLI argument parser                                          |

---

## Prior Art

Origin of this project started when I wrote [repogen](https://www.npmjs.com/package/repo-genesis-cli). While this is still used internally, it's outdated & lacks features. We ended up using submodules last year and remembered some of [the pain](https://www.youtube.com/watch?v=RFcc-BQjCsE) associated with an otherwise convenient feature. Ultimately wanted something that would allow a team of devs to have a consistent directory structure without being beholden to the all-mighty monolith

There are number of notable options in the monolith/package management space, ranging from pure workspaces to interactive build systems

Few notable examples:

- https://nx.dev/
- https://lerna.js.org/
- https://turbo.build/repo
- [git submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules)
- [VSCode workspaces](https://code.visualstudio.com/docs/editor/workspaces)

---

## Objectives

### Objectives::`MVP`
- Core commands (`init` and `sync`) work

### Objectives::`Post-MVP`
- Update README
  - install via `cargo`
  - Add support for install via `brew`

- Initialize
  - Generate a config
    - allow selecting config language
      - js (default)
      - json
      - yaml
  - Parse config that...
    - uses a SSH config provider to work with SSH connections
    - allows people to designate package manager's path per sub-repo (eg. pnpm, cargo, etc)
  - Generate initial gitignore (or update existing) to include all sub-repos paths (to avoid syncing symlinks or cloned `.repos` directory)
  - Clone repositories to hidden `.repo` directory
  - Load all submodules for repos
    > More info [here](https://docs.rs/git2/latest/git2/struct.Repository.html#method.submodules) 
  - Make the CLI interactive (during file generation)

- Maintain
  - One `brew doctor` equivalent command that can be used to do everything
  - Sync gitignore to match directories within `.repo` 
    >   eg. "foo", "bar" repos cloned. Auto-added to .gitignore file
  - Fetch updates for all sub-repos
    >   eg. `git pull origin master` on all nested repos
  - Enable idempotent fetches (to allow re-running repogen without needing to remove previously cloned repos or generated symlinks)
  - Enable cleaning up all local repos
  - Enable cleaning up all local symlinks
  - Enable generating alias files for nested tree structure
    > for now print out and allow them to source it
    - add prefix option in config
      - `${prefix}/${pwd}`
    - add shell output options
      - zsh
      - nushell

- Administrate
  - Create repo labels automagically ()
    >   eg. "foo", "bar" repos cloned. `${opt-label-prefix}-${reponame}-${opt-label-suffix}` generated on Github and Gitea
  - Show visual of dependency graph of repo structure
