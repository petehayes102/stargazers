use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlite::{Connection, State, Value};

const CREATE_TABLES: &str = "
    PRAGMA foreign_keys = ON;
    CREATE TABLE user (
        id INTEGER PRIMARY KEY,
        username VARCHAR UNIQUE,
        name VARCHAR,
        email VARCHAR
    );
    CREATE TABLE repository (
        id INTEGER PRIMARY KEY,
        full_name VARCHAR UNIQUE,
        stargazers INTEGER NOT NULL
    );
    CREATE TABLE user_repos (
        user INTEGER NOT NULL,
        repository INTEGER NOT NULL,
        type VARCHAR NOT NULL,
        date TEXT,
        FOREIGN KEY (user) REFERENCES user (id),
        FOREIGN KEY (repository) REFERENCES repository (id),
        UNIQUE(user, repository, type)
    );
    CREATE TABLE user_users (
        subject INTEGER NOT NULL,
        linked INTEGER NOT NULL,
        type VARCHAR NOT NULL,
        FOREIGN KEY (subject) REFERENCES user (id),
        FOREIGN KEY (linked) REFERENCES user (id),
        UNIQUE(subject, linked, type)
    );
";

pub fn init<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let exists = path.as_ref().exists();

    let conn = sqlite::open(path)?;

    if !exists {
        conn.execute(CREATE_TABLES)
            .context("Failed to create database tables")?;
    }

    Ok(conn)
}

pub fn has_user(conn: &Connection, id: i64) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT id FROM user where id = ?")?;
    stmt.bind((1, id))?;
    Ok(matches!(stmt.next(), Ok(State::Row)))
}

pub fn add_user(
    conn: &Connection,
    id: i64,
    username: &str,
    name: Option<&str>,
    email: Option<&str>,
) -> Result<()> {
    let mut stmt =
        conn.prepare("INSERT INTO user (id, username, name, email) VALUES (?, ?, ?, ?)")?;
    stmt.bind::<&[(_, Value)]>(&[
        (1, id.into()),
        (2, username.into()),
        (3, name.into()),
        (4, email.into()),
    ])?;
    while stmt.next()? == State::Row {}
    Ok(())
}

pub fn has_repo(conn: &Connection, id: i64) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT id FROM repository where id = ?")?;
    stmt.bind((1, id))?;
    Ok(matches!(stmt.next(), Ok(State::Row)))
}

pub fn add_repo(conn: &Connection, id: i64, full_name: &str, stargazers: i64) -> Result<()> {
    let mut stmt =
        conn.prepare("INSERT INTO repository (id, full_name, stargazers) VALUES (?, ?, ?)")?;
    stmt.bind::<&[(_, Value)]>(&[
        (1, id.into()),
        (2, full_name.into()),
        (3, stargazers.into()),
    ])?;
    while stmt.next()? == State::Row {}
    Ok(())
}

pub fn add_stargazer(conn: &Connection, user: i64, repo: i64, date: &DateTime<Utc>) -> Result<()> {
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO user_repos (user, repository, type, date) VALUES (?, ?, 'stargazer', ?)",
    )?;
    stmt.bind::<&[(_, Value)]>(&[
        (1, user.into()),
        (2, repo.into()),
        (3, date.to_string().into()),
    ])?;
    while stmt.next()? == State::Row {}
    Ok(())
}

pub fn add_subscriber(conn: &Connection, user: i64, repo: i64) -> Result<()> {
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO user_repos (user, repository, type) VALUES (?, ?, 'subscriber')",
    )?;
    stmt.bind::<&[(_, Value)]>(&[(1, user.into()), (2, repo.into())])?;
    while stmt.next()? == State::Row {}
    Ok(())
}

pub fn add_follower(conn: &Connection, subject: i64, linked: i64) -> Result<()> {
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO user_users (subject, linked, type) VALUES (?, ?, 'follower')",
    )?;
    stmt.bind::<&[(_, Value)]>(&[(1, subject.into()), (2, linked.into())])?;
    while stmt.next()? == State::Row {}
    Ok(())
}
