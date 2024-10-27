use crate::graphql_json;
use crate::parameters::Paramaters;
use gql_client::Client;
use std::collections::HashMap;
use std::io::{self, Write};

const QUERY: &str = r#"
query prs($owner: String!, $name: String!, $after: String) {
  repository(owner: $owner, name: $name) {
    pullRequests(
      states: [MERGED]
      first: 100,
      after: $after,
      orderBy: {direction: DESC, field: CREATED_AT}
    ) {
      totalCount
      pageInfo {
              endCursor
              startCursor
              hasNextPage
            }
      nodes {
        mergedAt
        createdAt
        #number
        author {
          login
        }
        reviews(first: 100, states: [COMMENTED, APPROVED]) {
          #totalCount
          nodes {
            author {
              login
            }
            state
            submittedAt
          }
        }
      }
    }
  }
}
"#;

pub async fn download_pr_data(params: &Paramaters) -> Vec<graphql_json::PRNode> {
    let endpoint = "https://api.github.com/graphql";

    println!(
        "\nRetrieving GitHub stats for pull requests for {}/{} form the last {days} days.",
        params.owner,
        params.repo,
        days = params.days_ago
    );

    let mut headers = HashMap::new();
    headers.insert("Authorization", format!("Bearer {}", params.token));
    headers.insert("User-Agent", "gql-client".to_string());

    let client = Client::new_with_headers(endpoint, headers);

    let mut vars = graphql_json::Vars {
        owner: params.owner.clone(),
        name: params.repo.clone(),
        after: None,
    };

    let mut nodes: Vec<graphql_json::PRNode> = Vec::new();

    let cutoff_date = Some(chrono::Utc::now() - chrono::Duration::days(params.days_ago));

    let mut page_count = 1;
    loop {
        print!("\r ... fetching 100 records from page {page_count}");
        let _ = io::stdout().flush(); // Don't care if this fails

        let data = client
            .query_with_vars_unwrap::<graphql_json::Data, graphql_json::Vars>(
                QUERY,
                graphql_json::Vars {
                    owner: vars.owner.clone(),
                    name: vars.name.clone(),
                    after: vars.after.clone(),
                },
            )
            .await
            .unwrap();

        page_count += 1;

        let filtered_nodes: Vec<graphql_json::PRNode> = data
            .repository
            .pull_requests
            .nodes
            .into_iter()
            // Ignore records that are too old
            .filter(|pr| pr.created_at > cutoff_date)
            .collect();

        // Since we filter out old records and sort by created_at, we can break early if we didn't get any new records
        if filtered_nodes.len() == 0 {
            break;
        }

        nodes.extend(filtered_nodes);

        // Maybe we got new records but there are no more PRs to fetch
        let has_next_page = data.repository.pull_requests.page_info.has_next_page;
        let end_cursor = data.repository.pull_requests.page_info.end_cursor.clone();
        if !has_next_page || end_cursor.is_none() {
            break;
        }

        vars.after = end_cursor;
    }

    // Clear the line
    println!("\r                                                  \r");

    nodes
}
