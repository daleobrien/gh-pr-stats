use gql_client::Client;
use std::collections::HashMap;
use std::collections::HashSet;
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};
extern crate chrono;

mod graphql_json;
mod parameters;

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

fn normalize(author_pr_created: HashMap<String, i32>, total_prs: u32) -> HashMap<String, f32> {
    author_pr_created
        .into_iter()
        .filter(|(_, v)| (*v as f32) / (total_prs as f32) > 0.0)
        .map(|(k, v)| (k, v as f32))
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = parameters::Paramaters::new();
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
        owner: params.owner,
        name: params.repo,
    };
    let data = client
        .query_with_vars_unwrap::<graphql_json::Data, graphql_json::Vars>(QUERY, vars)
        .await
        .unwrap();

    let mut author_pr_created = HashMap::new();
    let mut author_pr_approved = HashMap::new();
    let mut author_pr_comments = HashMap::new();

    data.repository.pull_requests.nodes.iter().for_each(|pr| {
        let author = &pr.author.login;

        // Ignore the author if they are in the ignored list
        if params.ignored_users.contains(&author) {
            return;
        }

        let count = author_pr_created.entry(author.clone()).or_insert(0);
        *count += 1;

        let mut seen_reviewer_for_state = HashSet::new();

        pr.reviews.nodes.iter().for_each(|review| {
            let reviewer = &review.author.login;

            // Ignore the reviewer if they are the author as well, e.g. they commentef on their own PR
            if reviewer == author {
                return;
            }

            // Ignore the reviewer if they are in the ignored list
            if params.ignored_users.contains(&reviewer) {
                return;
            }

            // Don't count the same reviewer twice for the same thing
            if !seen_reviewer_for_state.contains(&(reviewer.clone(), review.state.clone())) {
                seen_reviewer_for_state.insert((reviewer.clone(), review.state.clone()));

                if review.state == "APPROVED" {
                    let count = author_pr_approved.entry(reviewer.clone()).or_insert(0);
                    *count += 1;
                }
                if review.state == "COMMENTED" {
                    let count = author_pr_comments.entry(reviewer.clone()).or_insert(0);
                    *count += 1;
                }
            }
        });
    });

    let mut all_users = HashSet::new();
    all_users.extend(author_pr_created.keys().cloned());
    all_users.extend(author_pr_approved.keys().cloned());
    all_users.extend(author_pr_comments.keys().cloned());

    let mut all_users: Vec<String> = all_users.into_iter().collect();
    all_users.sort();

    // Fill in the blanks for each user
    all_users.iter().for_each(|user| {
        if !author_pr_created.contains_key(user) {
            author_pr_created.insert(user.clone(), 0);
        }
        if !author_pr_approved.contains_key(user) {
            author_pr_approved.insert(user.clone(), 0);
        }
        if !author_pr_comments.contains_key(user) {
            author_pr_comments.insert(user.clone(), 0);
        }
    });

    let mut builder = Builder::default();
    let header = vec!["User", "% PRs", "% Approved", "% Comments"];
    builder.push_record(header);

    let total_prs = author_pr_created.values().sum::<i32>();
    print!("Found a total of {} PRs", total_prs);
    if params.ignored_users.len() > 0 {
        print!(" (with filters applied)",);
    }
    println!("");

    for user in &all_users {
        let pr_count = author_pr_created.get(user).unwrap();

        let pr_approved_count = author_pr_approved.get(user).unwrap();
        let pr_comment_count = author_pr_comments.get(user).unwrap();

        let pr_percentage = 100 * pr_count / total_prs;

        // Count one's own prs as approved and commented on, keeps the math simple
        let approved_percentage = 100 * (pr_count + pr_approved_count) / total_prs;
        let comment_percentage = 100 * (pr_count + pr_comment_count) / total_prs;

        let row = vec![
            user.clone(),
            pr_percentage.to_string(),
            approved_percentage.to_string(),
            comment_percentage.to_string(),
        ];
        builder.push_record(row);
    }

    let table = builder
        .build()
        .with(Style::rounded())
        .modify(Rows::new(1..), Alignment::right())
        .to_string();

    println!("{table}");

    Ok(())
}
