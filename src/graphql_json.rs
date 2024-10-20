use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Vars {
    pub(crate) owner: String,
    pub(crate) name: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub(crate) repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    #[serde(rename = "pullRequests")]
    pub(crate) pull_requests: PullRequests,
}

#[derive(Deserialize, Debug)]
pub struct PullRequests {
    // #[serde(rename = "totalCount")]
    // pub(crate) total_count: u32,
    pub(crate) nodes: Vec<PRNode>,
}

#[derive(Deserialize, Debug)]
pub struct PRNode {
    // #[serde(rename = "mergedAt")]
    // pub(crate) merged_at: String,
    // pub(crate) number: u32,
    pub(crate) author: Author,
    pub(crate) reviews: Reviews,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    #[serde(rename = "login")]
    pub(crate) login: String,
}

#[derive(Deserialize, Debug)]
pub struct Reviews {
    // #[serde(rename = "totalCount")]
    // pub(crate) total_count: u32,
    pub(crate) nodes: Vec<ReviewNode>,
}

#[derive(Deserialize, Debug)]
pub struct ReviewNode {
    pub(crate) state: String,
    // #[serde(rename = "submittedAt")]
    // pub(crate) submitted_at: String,
    pub(crate) author: Author,
}
