use crate::parameters::Paramaters;
use std::collections::HashMap;
use tabled::builder::Builder;
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Style};

pub fn pretty_print(
    params: Paramaters,
    author_pr_created: HashMap<String, u32>,
    author_pr_approved: HashMap<String, u32>,
    author_pr_comments: HashMap<String, u32>,
    all_users: &Vec<String>,
) {
    let mut builder = Builder::default();
    let header = vec!["User", "% PRs", "% Approved", "% Comments"];
    builder.push_record(header);

    let total_prs = author_pr_created.values().sum::<u32>();
    print!("Found a total of {} PRs", total_prs);
    if params.ignored_users.len() > 0 {
        print!(" (with filters applied)",);
    }
    println!("");

    for user in all_users {
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
}
