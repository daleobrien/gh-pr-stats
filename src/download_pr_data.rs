use crate::graphql_json;
use crate::graphql_json::Data;
use crate::parameters::Paramaters;
use gql_client::Client;
use std::collections::HashMap;

const QUERY: &str = r#"
query prs($owner: String!, $name: String!) {
  repository(owner: $owner, name: $name) {
    pullRequests(
      states: [MERGED]
      first: 100
      orderBy: {direction: DESC, field: CREATED_AT}
    ) {
      totalCount
      nodes {
        mergedAt
        number
        author {
          login
        }
        reviews(first: 100, states: [COMMENTED, APPROVED]) {
          totalCount
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

pub async fn download_pr_data(params: &Paramaters) -> Data {
    let endpoint = "https://api.github.com/graphql";

    println!(
        "\nRetrieving GitHub stats from up to the last 100 pull requests for {}/{} ...\n",
        params.owner, params.repo
    );

    let mut headers = HashMap::new();
    headers.insert("Authorization", format!("Bearer {}", params.token));
    headers.insert("User-Agent", "gql-client".to_string());

    let client = Client::new_with_headers(endpoint, headers);

    let vars = graphql_json::Vars {
        owner: params.owner.clone(),
        name: params.repo.clone(),
    };

    client
        .query_with_vars_unwrap::<graphql_json::Data, graphql_json::Vars>(QUERY, vars)
        .await
        .unwrap()
}
