use std::{fs::File, io::Write, path::Path};

use anyhow::Result;
use sqlite::Connection;

use crate::db::get_top_repos;

pub async fn analyse<P: AsRef<Path>>(db: &Connection, path: P) -> Result<()> {
    let repos = get_top_repos(db, 20)?;
    let json = serde_json::to_string(&repos)?;

    let mut fh = File::create(path)?;
    fh.write_all(json.as_bytes())?;

    Ok(())
}
