use crate::graphql_json::Data;
use crate::parameters::Paramaters;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Hash, Eq, PartialEq, EnumIter)]
pub enum DataType {
    Approved,
    Commented,
    Created,
}


pub fn parse_data(
    params: &Paramaters,
    data: Data,
) -> (HashMap<DataType, HashMap<String, u32>>, Vec<String>) {
    let mut user_data: HashMap<DataType, HashMap<String, u32>> = HashMap::new();

    for data_type in DataType::iter() {
        user_data.insert(data_type, HashMap::new());
    }

    data.repository.pull_requests.nodes.iter().for_each(|pr| {
        let author = &pr.author.login;

        // Ignore the author if they are in the ignored list
        if params.ignored_users.contains(author) {
            return;
        }

        // Increment the PR count for the author
        *user_data
            .get_mut(&DataType::Created)
            .unwrap()
            .entry(author.clone())
            .or_insert(0) += 1;

        let mut seen_reviewer_for_state = HashSet::new();

        // Iterate over the reviews for the PR
        pr.reviews.nodes.iter().for_each(|review| {
            let reviewer = &review.author.login;

            // Ignore the reviewer if they are the author as well, e.g. they commented on their own PR
            if reviewer == author {
                return;
            }

            // Ignore the reviewer if they are in the ignored list
            if params.ignored_users.contains(reviewer) {
                return;
            }

            // Don't count the same reviewer twice for the same thing
            if !seen_reviewer_for_state.contains(&(reviewer.clone(), review.state.clone())) {
                seen_reviewer_for_state.insert((reviewer.clone(), review.state.clone()));

                let data_type_for_state: DataType = match review.state.as_str() {
                    "APPROVED" => DataType::Approved,
                    "COMMENTED" => DataType::Commented,
                    _ => {
                        return;
                    }
                };

                *user_data
                    .get_mut(&data_type_for_state)
                    .unwrap()
                    .entry(reviewer.clone())
                    .or_insert(0) += 1;
            }
        });
    });

    // Get all users
    let mut all_users = HashSet::new();
    for data_type in DataType::iter() {
        all_users.extend(user_data.get(&data_type).unwrap().keys().cloned());
    }
    let mut all_users: Vec<String> = all_users.into_iter().collect();
    all_users.sort();

    // Fill in the blanks for each user
    all_users.iter().for_each(|user| {
        for data_type in DataType::iter() {
            if !user_data.get(&data_type).unwrap().contains_key(user) {
                user_data
                    .get_mut(&data_type)
                    .unwrap()
                    .insert(user.clone(), 0);
            }
        }
    });

    (user_data, all_users)
}
