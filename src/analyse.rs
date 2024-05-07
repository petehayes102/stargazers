use anyhow::Result;
use rocket::{get, routes, serde::json::Json};
use rocket_db_pools::{sqlx, Connection, Database};

use crate::db::get_top_repos;

#[derive(Database)]
#[database("stargazers")]
pub(crate) struct Stargazers(sqlx::SqlitePool);

pub async fn analyse(open: bool) -> Result<()> {
    rocket::build()
        .mount("/repos/top", routes![top_repos])
        .launch()
        .await?;

    Ok(())
}

#[get("/repos/top/<num>", format = "json")]
async fn top_repos(
    mut db: Connection<Stargazers>,
    num: u32,
) -> std::result::Result<Json<Vec<(String, u32)>>, String> {
    match get_top_repos(&mut db, num).await {
        Ok(repos) => Ok(Json(repos)),
        Err(e) => Err(e.to_string()),
    }
}
