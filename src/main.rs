mod download_pr_data;
mod graphql_json;
mod parameters;
mod parse;
mod pretty_print;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = parameters::Paramaters::new();

    let data = download_pr_data::download_pr_data(&params).await;

    let (user_data, all_users, user_to_user_pr_count) = parse::parse_data(&params, data);

    pretty_print::print_data_as_table(params, user_data, &all_users, user_to_user_pr_count);

    // TODO: It would be nice to create a table the shows the number of % PRs approved by each user against each user
    Ok(())
}
