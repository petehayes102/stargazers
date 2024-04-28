use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use github::{create_client, get_repository};

mod db;
mod github;

/// Fetch and analyse GitHub stargazers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Owner name
    #[arg(short, long)]
    owner: String,

    /// Repository name
    #[arg(short, long)]
    repo: String,

    /// Personal access token
    #[arg(short, long)]
    pat: String,

    /// Path to database file
    #[arg(short, long, default_value = "stargazers.db")]
    db: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let db = db::init(&args.db)?;

    let client = create_client(&args.pat)?;
    let repo = get_repository(&client, &args.owner, &args.repo).await?;

    if !db::has_repo(&db, repo.id as i64)? {
        println!("Adding repo: {}", repo.full_name);

        db::add_repo(
            &db,
            repo.id as i64,
            &repo.full_name,
            repo.stargazers_count as i64,
        )?;
    }

    for s in repo.get_stargazers(&client).await? {
        println!("Adding user: {}", s.user.login);

        if !db::has_user(&db, s.user.id as i64)? {
            db::add_user(
                &db,
                s.user.id as i64,
                &s.user.login,
                s.user.name.as_deref(),
                s.user.email.as_deref(),
            )?;
        }

        db::add_stargazer(&db, s.user.id as i64, repo.id as i64, &s.starred_at)?;

        // Get followers
        for u in s.user.get_followers(&client).await? {
            println!("Adding follower of {}: {}", s.user.login, u.login);

            if !db::has_user(&db, u.id as i64)? {
                db::add_user(
                    &db,
                    u.id as i64,
                    &u.login,
                    u.name.as_deref(),
                    u.email.as_deref(),
                )?;
            }

            db::add_follower(&db, s.user.id as i64, u.id as i64)?;
        }

        // Get starred repos
        for r in s.user.get_starred(&client).await? {
            println!(
                "Adding starred repo of {}: {}",
                s.user.login, r.repo.full_name
            );

            if !db::has_repo(&db, r.repo.id as i64)? {
                db::add_repo(
                    &db,
                    r.repo.id as i64,
                    &r.repo.full_name,
                    r.repo.stargazers_count as i64,
                )?;
            }

            db::add_stargazer(&db, s.user.id as i64, r.repo.id as i64, &r.starred_at)?;
        }

        // Get subscribed repos
        for r in s.user.get_subscribed(&client).await? {
            println!("Adding starred repo of {}: {}", s.user.login, r.full_name);

            if !db::has_repo(&db, r.id as i64)? {
                db::add_repo(&db, r.id as i64, &r.full_name, r.stargazers_count as i64)?;
            }

            db::add_subscriber(&db, s.user.id as i64, r.id as i64)?;
        }
    }

    Ok(())
}
