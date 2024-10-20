use std::env;
use std::fmt::Error;

pub(crate) fn env_var_to_string(env_name: &str) -> Result<String, Error> {
    let owner = env::var(env_name);
    if owner.is_err() {
        return Err(Error);
    }
    Ok(owner.unwrap())
}

pub(crate) struct Paramaters {
    pub(crate) owner: String,
    pub(crate) repo: String,
    pub(crate) token: String,
    pub(crate) ignored_users: Vec<String>,
}

impl Paramaters {
    pub(crate) fn new() -> Paramaters {
            let owner = env_var_to_string("GITHUB_OWNER");
            let repo = env_var_to_string("GITHUB_REPO");
            let token = env_var_to_string("GITHUB_TOKEN");
            let ignored_users: Vec<String> = env::var("IGNORED_USERS")
                .unwrap_or_default()
                .split(',')
                .map(String::from)
                .collect();

            if owner.is_err() || repo.is_err() || token.is_err() {
                panic!("GITHUB_OWNER, GITHUB_REPO and GITHUB_TOKEN must be set");
            }

            Paramaters {
                owner: owner.unwrap(),
                repo: repo.unwrap(),
                token: token.unwrap(),
                ignored_users,
            }
        }
}
