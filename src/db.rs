use anyhow::Result;
use chrono::{DateTime, Utc};
use rocket_db_pools::Connection;
use sqlx::{query, Pool, Row, Sqlite};

use crate::analyse::Stargazers;

pub async fn init(path: &str) -> Result<Pool<Sqlite>> {
    let pool = Pool::<Sqlite>::connect(&path).await?;
    sqlx::migrate!("db/migrations/").run(&pool).await?;
    Ok(pool)
}

pub async fn has_user(conn: &Pool<Sqlite>, id: i64) -> Result<bool> {
    Ok(query("SELECT id FROM user where id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await?
        .is_some())
}

pub async fn add_user(
    conn: &Pool<Sqlite>,
    id: i64,
    username: &str,
    name: Option<&str>,
    email: Option<&str>,
) -> Result<()> {
    query("INSERT INTO user (id, username, name, email) VALUES (?, ?, ?, ?)")
        .bind(id)
        .bind(username)
        .bind(name)
        .bind(email)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn has_repo(conn: &Pool<Sqlite>, id: i64) -> Result<bool> {
    Ok(query("SELECT id FROM repository where id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await?
        .is_some())
}

pub async fn add_repo(
    conn: &Pool<Sqlite>,
    id: i64,
    full_name: &str,
    stargazers: i64,
) -> Result<()> {
    query("INSERT INTO repository (id, full_name, stargazers) VALUES (?, ?, ?)")
        .bind(id)
        .bind(full_name)
        .bind(stargazers)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn add_stargazer(
    conn: &Pool<Sqlite>,
    user: i64,
    repo: i64,
    date: &DateTime<Utc>,
) -> Result<()> {
    query("INSERT OR REPLACE INTO user_repos (user, repository, type, date) VALUES (?, ?, 'stargazer', ?)")
        .bind(user)
        .bind(repo)
        .bind(date.to_string())
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn add_subscriber(conn: &Pool<Sqlite>, user: i64, repo: i64) -> Result<()> {
    query("INSERT OR REPLACE INTO user_repos (user, repository, type) VALUES (?, ?, 'subscriber')")
        .bind(user)
        .bind(repo)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn add_follower(conn: &Pool<Sqlite>, subject: i64, linked: i64) -> Result<()> {
    query("INSERT OR REPLACE INTO user_users (subject, linked, type) VALUES (?, ?, 'follower')")
        .bind(subject)
        .bind(linked)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_top_repos(
    conn: &mut Connection<Stargazers>,
    limit: u32,
) -> Result<Vec<(String, u32)>> {
    Ok(query(
        "SELECT full_name, count(*) as count
        FROM repository r
        LEFT JOIN user_repos ur ON (r.id = ur.repository AND ur.type = 'stargazer')
        GROUP BY ur.repository
        ORDER BY count DESC
        LIMIT ?",
    )
    .bind(limit)
    .fetch_all(&mut ***conn)
    .await?
    .into_iter()
    .map(|row| (row.get("full_name"), row.get("count")))
    .collect())
}
