use std::collections::{HashMap, HashSet};
use crate::graphql_json::Data;
use crate::parameters::Paramaters;

pub fn parse_data(params: &Paramaters, data: Data) -> (HashMap<String, u32>, HashMap<String, u32>, HashMap<String, u32>, Vec<String>) {

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
    (author_pr_created, author_pr_approved, author_pr_comments, all_users)
}