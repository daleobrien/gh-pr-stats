use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Vars {
    pub(crate) owner: String,
    pub(crate) name: String,
    pub(crate) after: Option<String>,
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
    pub(crate) nodes: Vec<PRNode>,
    #[serde(rename = "pageInfo")]
    pub(crate) page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct PRNode {
    #[serde(rename = "createdAt")]
    pub(crate) created_at: Option<DateTime<Utc>>,
    // #[serde(rename = "mergedAt")]
    // pub(crate) merged_at: Option<DateTime<Utc>>,
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
    // pub(crate) submitted_at: Option<DateTime<Utc>>,
    pub(crate) author: Author,
}

#[derive(Deserialize, Debug)]
pub struct PageInfo {
    #[serde(rename = "endCursor")]
    pub(crate) end_cursor: Option<String>,
    // #[serde(rename = "startCursor")]
    // pub(crate) start_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub(crate) has_next_page: bool,
}
