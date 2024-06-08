use std::path::{Path, PathBuf};

use anyhow::Result;
use rocket::{
    fs::{relative, NamedFile},
    get, routes,
    serde::json::Json,
};
use rocket_db_pools::{sqlx, Connection, Database};

use crate::db::get_top_repos;

#[derive(Database)]
#[database("stargazers")]
pub(crate) struct Stargazers(sqlx::SqlitePool);

pub async fn analyse(open: bool) -> Result<()> {
    let server = rocket::build()
        .mount("/", routes![static_web, top_repos])
        .attach(Stargazers::init())
        .ignite()
        .await?;

    if open {
        let http = if server.config().tls_enabled() {
            "https"
        } else {
            "http"
        };
        webbrowser::open(&format!(
            "{http}://{}:{}",
            server.config().address,
            server.config().port
        ))
        .unwrap();
    }

    server.launch().await?;

    Ok(())
}

#[get("/<path..>")]
async fn static_web(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(relative!("web/dist")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
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
