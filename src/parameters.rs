use std::env;
use std::fmt::Error;

const DEFAULT_DAYS_AGO: i64 = 60;

pub(crate) fn env_var_to_string(env_name: &str) -> Result<String, Error> {
    let owner = env::var(env_name);
    if owner.is_err() {
        return Err(Error);
    }
    Ok(owner.unwrap())
}

pub(crate) fn env_var_to_number(env_name: &str) -> i64 {
    let days_ago = env::var(env_name);
    if days_ago.is_err() {
        return DEFAULT_DAYS_AGO;
    }
    days_ago.unwrap().parse::<i64>().unwrap()
}

pub(crate) struct Paramaters {
    pub(crate) owner: String,
    pub(crate) repo: String,
    pub(crate) token: String,
    pub(crate) ignored_users: Vec<String>,
    pub(crate) days_ago: i64,
}

impl Paramaters {
    pub(crate) fn new() -> Paramaters {
            let days_ago = env_var_to_number("GITHUB_STATS_DAYS_AGO");
            let owner = env_var_to_string("GITHUB_OWNER");
            let repo = env_var_to_string("GITHUB_REPO");
            let token = env_var_to_string("GITHUB_TOKEN");
            let ignored_users: Vec<String> = env::var("GITHUB_STATS_IGNORED_USERS")
                .unwrap_or_else(|_| "codescene-delta-analysis,coderabbitai".to_string())
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
                days_ago,
            }
        }
}
