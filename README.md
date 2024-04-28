# Stargazers

Analytics tool for GitHub stargazers, using SQLite as a backend.

Note: This is currently under development. The code will download all data to SQLite but no frontend exists to analyse/display the data.

## Usage

```bash
$ cargo run -- --help  # Show usage
$ cargo run            # Fetch all data from GitHub
```

## Analytics

You can query the database using the SQLite client:

```
$ sqlite3 stargazers.db
```

Here's an example query to get the top 20 starred repos for your stargazers. This tells us what else your stargazers are interested in:

```sql
SELECT full_name, count(*) as count
FROM repository r
LEFT JOIN user_repos ur ON (r.id = ur.repository AND ur.type = 'stargazer')
GROUP BY ur.repository
ORDER BY count DESC
LIMIT 20;
```

## To do

1. Build a front end to analyse and display SQLite results
2. Abstract database using an ORM
3. Tests
4. Improve GitHub binding (e.g. autogenerate structs from GitHub schemas, parallelise pagination etc., publish as new lib on [crates.io]())
5. Improve data collection (e.g. collect commits on target repo to correlate with stargazer dates)
