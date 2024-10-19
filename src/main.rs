use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Style},
};
mod graphql_json;
mod parameters;
mod download_pr_data;
mod parse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = parameters::Paramaters::new();

    let data = download_pr_data::download_pr_data(&params).await;

    let (author_pr_created, author_pr_approved, author_pr_comments, all_users) = parse::parse_data(&params, data);

    let mut builder = Builder::default();
    let header = vec!["User", "% PRs", "% Approved", "% Comments"];
    builder.push_record(header);

    let total_prs = author_pr_created.values().sum::<u32>();
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

