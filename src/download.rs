use anyhow::Result;
use sqlite::Connection;

use crate::{
    db,
    github::{create_client, get_repository},
};

pub async fn download(
    db: &Connection,
    pat: &str,
    owner: &str,
    repo: &str,
    quick: bool,
) -> Result<()> {
    let client = create_client(pat)?;
    let repo = get_repository(&client, owner, repo).await?;

    if !db::has_repo(db, repo.id as i64)? {
        println!("Adding repo: {}", repo.full_name);

        db::add_repo(
            db,
            repo.id as i64,
            &repo.full_name,
            repo.stargazers_count as i64,
        )?;
    }

    for s in repo.get_stargazers(&client).await? {
        if !db::has_user(db, s.user.id as i64)? {
            println!("Adding user: {}", s.user.login);

            db::add_user(
                db,
                s.user.id as i64,
                &s.user.login,
                s.user.name.as_deref(),
                s.user.email.as_deref(),
            )?;
        } else if quick {
            println!("Skipping user: {}", s.user.login);
            continue;
        }

        db::add_stargazer(db, s.user.id as i64, repo.id as i64, &s.starred_at)?;

        // Get followers
        for u in s.user.get_followers(&client).await? {
            println!("Adding follower of {}: {}", s.user.login, u.login);

            if !db::has_user(db, u.id as i64)? {
                db::add_user(
                    db,
                    u.id as i64,
                    &u.login,
                    u.name.as_deref(),
                    u.email.as_deref(),
                )?;
            }

            db::add_follower(db, s.user.id as i64, u.id as i64)?;
        }

        // Get starred repos
        for r in s.user.get_starred(&client).await? {
            println!(
                "Adding starred repo of {}: {}",
                s.user.login, r.repo.full_name
            );

            if !db::has_repo(db, r.repo.id as i64)? {
                db::add_repo(
                    db,
                    r.repo.id as i64,
                    &r.repo.full_name,
                    r.repo.stargazers_count as i64,
                )?;
            }

            db::add_stargazer(db, s.user.id as i64, r.repo.id as i64, &r.starred_at)?;
        }

        // Get subscribed repos
        for r in s.user.get_subscribed(&client).await? {
            println!("Adding starred repo of {}: {}", s.user.login, r.full_name);

            if !db::has_repo(db, r.id as i64)? {
                db::add_repo(db, r.id as i64, &r.full_name, r.stargazers_count as i64)?;
            }

            db::add_subscriber(db, s.user.id as i64, r.id as i64)?;
        }
    }

    Ok(())
}
