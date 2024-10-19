mod graphql_json;
mod parameters;
mod download_pr_data;
mod parse;
mod pretty_print;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = parameters::Paramaters::new();

    let data = download_pr_data::download_pr_data(&params).await;

    let (author_pr_created, author_pr_approved, author_pr_comments, all_users) = parse::parse_data(&params, data);

    pretty_print::pretty_print(params, author_pr_created, author_pr_approved, author_pr_comments, &all_users);

    Ok(())
}

