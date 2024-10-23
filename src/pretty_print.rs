use crate::parameters::Paramaters;
use crate::parse::DataType;
use std::collections::HashMap;
use tabled::builder::Builder;
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Style};

pub fn left_pad(s: &str, n: usize) -> String {
    format!("{:>width$}", s, width = n)
}

pub fn print_data_as_table(
    params: Paramaters,
    user_data: HashMap<DataType, HashMap<String, u32>>,
    all_users: &Vec<String>,
    user_to_user_pr_count: HashMap<(String, String), u32>,
) {
    let mut stats_builder = Builder::default();
    let stats_header = vec!["User", "PRs [%]", "Approved [%]", "Comments [%]"];
    let max_len = stats_header.iter().map(|u| u.len()).max().unwrap();
    let stats_header = stats_header
        .iter()
        .map(|u| left_pad(u, max_len))
        .collect::<Vec<String>>();
    stats_builder.push_record(stats_header);

    // Find the longest username for formatting
    let max_user_len = all_users.iter().map(|u| u.len()).max().unwrap();

    let mut relationship_builder = Builder::default();
    let mut relationship_header:Vec<String> = vec!["Reviewer\\Author".to_string()];
    let header = all_users.iter().map(|u| left_pad(u, max_user_len)).collect::<Vec<String>>();
    relationship_header.extend(header);
    relationship_builder.push_record(relationship_header);

    let total_prs = user_data
        .get(&DataType::Created)
        .unwrap()
        .values()
        .sum::<u32>();
    print!("Found a total of {} PRs", total_prs);
    if !params.ignored_users.is_empty() {
        print!(" (with filters applied)",);
    }
    println!("");

    for user in all_users {
        let pr_created_n = user_data
            .get(&DataType::Created)
            .unwrap()
            .get(user)
            .unwrap();
        let pr_approved_n = user_data
            .get(&DataType::Approved)
            .unwrap()
            .get(user)
            .unwrap();
        let pr_comment_n = user_data
            .get(&DataType::Commented)
            .unwrap()
            .get(user)
            .unwrap();

        let pr_percentage = 100 * pr_created_n / total_prs;

        // Count one's own prs as approved and commented on, keeps the math simple
        let approved_percentage = 100 * (pr_created_n + pr_approved_n) / total_prs;
        let comment_percentage = 100 * (pr_created_n + pr_comment_n) / total_prs;

        let row = vec![
            user.clone(),
            pr_percentage.to_string(),
            approved_percentage.to_string(),
            comment_percentage.to_string(),
        ];
        stats_builder.push_record(row);

        let mut relationship_row = vec![user.to_string()];
        for user2 in all_users {
            let c = user_to_user_pr_count.get(&(user.clone(), user2.clone()));
            if user == user2 {
                relationship_row.push("-".to_string());
                continue;
            }
            relationship_row.push(c.unwrap_or(&0).to_string())
        }
        relationship_builder.push_record(relationship_row);
    }

    println!("\nStatistics:");
    let table = stats_builder
        .build()
        .with(Style::rounded())
        .modify(Rows::new(1..), Alignment::right())
        .to_string();
    println!("{table}");

    println!("\nRelationships [count]:");
    let table = relationship_builder
        .build()
        .with(Style::rounded())
        .modify(Rows::new(1..), Alignment::right())
        .to_string();
    println!("{table}");
}
