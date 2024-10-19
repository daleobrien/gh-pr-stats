use crate::parameters::Paramaters;
use std::collections::HashMap;
use tabled::builder::Builder;
use tabled::settings::object::Rows;
use tabled::settings::{Alignment, Style};
use crate::parse::DataType;

pub fn print_data_as_table(
    params: Paramaters,
    user_data: HashMap<DataType, HashMap<String, u32>>,
    all_users: &Vec<String>,
) {
    let mut builder = Builder::default();
    let header = vec!["User", "% PRs", "% Approved", "% Comments"];
    builder.push_record(header);

    let total_prs = user_data.get(&DataType::Created).unwrap().values().sum::<u32>();
    print!("Found a total of {} PRs", total_prs);
    if params.ignored_users.len() > 0 {
        print!(" (with filters applied)",);
    }
    println!("");

    for user in all_users {
        let pr_created_n = user_data.get(&DataType::Created).unwrap().get(user).unwrap();
        let pr_approved_n = user_data.get(&DataType::Approved).unwrap().get(user).unwrap();
        let pr_comment_n = user_data.get(&DataType::Commented).unwrap().get(user).unwrap();

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
        builder.push_record(row);
    }

    let table = builder
        .build()
        .with(Style::rounded())
        .modify(Rows::new(1..), Alignment::right())
        .to_string();

    println!("{table}");
}
