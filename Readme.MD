# Extract some statistical information from the last 100 PRs

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
│  sam.mckee  │    17 │         76 │         54 │
│     cindy13 │    22 │         77 │         52 │
│        jane │    27 │         76 │         58 │
│ longer.name │    17 │         42 │         27 │
│    shorter0 │    14 │         40 │         30 │
╰─────────────┴───────┴────────────┴────────────╯
```
