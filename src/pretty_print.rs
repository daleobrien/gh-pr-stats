use crate::parameters::Paramaters;
use crate::parse::DataType;
use std::collections::HashMap;
use tabled::builder::Builder;
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Style};

pub fn print_data_as_table(
    params: Paramaters,
    user_data: HashMap<DataType, HashMap<String, u32>>,
    all_users: &Vec<String>,
    user_to_user_pr_count: HashMap<(String, String), u32>,
) {
    let mut stats_builder = Builder::default();
    let stats_header = vec!["User", "% PRs", "% Approved", "% Comments"];
    stats_builder.push_record(stats_header);

    let mut relationship_builder = Builder::default();
    let mut relationship_header = vec!["Reviewer\\Author"];
    for user in all_users {
        relationship_header.push(user);
    }
    relationship_builder.push_record(relationship_header);

    let total_prs = user_data
        .get(&DataType::Created)
        .unwrap()
        .values()
        .sum::<u32>();
    print!("Found a total of {} PRs", total_prs);
    if params.ignored_users.len() > 0 {
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
        let mut relationship_header = vec![user.clone()];
        for user2 in all_users {
            let c = user_to_user_pr_count.get(&(user.clone(), user2.clone()));
            if user == user2 {
                relationship_header.push("".to_string());
                continue;
            }
            relationship_header.push(c.unwrap_or(&0).to_string())
        }
        relationship_builder.push_record(relationship_header);
    }

    println!("Statistics:");
    let table = stats_builder
        .build()
        .with(Style::rounded())
        .modify(Rows::new(1..), Alignment::right())
        .to_string();
    println!("{table}");

    println!("Relationships:");
    let table = relationship_builder
        .build()
        .with(Style::rounded())
        .modify(Rows::new(1..), Alignment::right())
        .to_string();
    println!("{table}");
}
