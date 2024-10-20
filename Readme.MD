# Extract some statistical information from the last 100 PRs

Quick project to determine the percentage of involvement for each contriber a given project. 

*I was wanting to know if each contriber of the team is looking at each other's PRs.*

# Installation

```bash
cargo build --release
```


## usage:
```bash
export GITHUB_TOKEN=your_github_token
export GITHUB_REPO=your_github_repo_name
export GITHUB_OWNER=your_github_owner_name

# optional list of github users/bots to ignore
export IGNORED_USERS='codescene-delta-analysis,coderabbitai'

cargo run

```

And it will print out the following information:
```text

Retrieving GitHub stats from up to the last 100 pull requests for Someone/Something ...

Found a total of 96 PRs (with filters applied)
╭─────────────┬───────┬────────────┬────────────╮
│ User        │ % PRs │ % Approved │ % Comments │
├─────────────┼───────┼────────────┼────────────┤
│   sam.mckee │    17 │         76 │         54 │
│     cindy13 │    22 │         77 │         52 │
│        jane │    27 │         76 │         58 │
│ longer.name │    17 │         42 │         27 │
│    shorter0 │    14 │         40 │         30 │
╰─────────────┴───────┴────────────┴────────────╯
```
Key:
* `% PRs`: The percentage of PRs did the user create
* `% Approved`: The percentage of PRs they have approved (1). 
* `% Comments`: The percentage of PRs they have commented on (1).

(1) The percent also includes the PRs they have created since they would consider them good to go.
