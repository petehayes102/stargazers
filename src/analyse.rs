use std::path::{Path, PathBuf};

use anyhow::Result;
use rocket::{
    fairing::{Fairing, Info, Kind},
    fs::{relative, NamedFile},
    get,
    http::Header,
    options, routes,
    serde::json::Json,
    Request, Response,
};
use rocket_db_pools::{sqlx, Connection, Database};

use crate::db::{get_top_following, get_top_repos, get_top_subscribed};

#[derive(Database)]
#[database("stargazers")]
pub(crate) struct Stargazers(sqlx::SqlitePool);

#[allow(dead_code)]
struct Cors;

pub async fn analyse(open: bool) -> Result<()> {
    let builder = rocket::build()
        .mount(
            "/",
            routes![
                static_web,
                top_repos,
                top_following,
                top_subscribed,
                all_options
            ],
        )
        .attach(Stargazers::init());

    #[cfg(debug_assertions)]
    let builder = builder.attach(Cors);

    let server = builder.ignite().await?;

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

#[get("/following/top/<num>", format = "json")]
async fn top_following(
    mut db: Connection<Stargazers>,
    num: u32,
) -> std::result::Result<Json<Vec<(String, u32)>>, String> {
    match get_top_following(&mut db, num).await {
        Ok(followed) => Ok(Json(followed)),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/subscribed/top/<num>", format = "json")]
async fn top_subscribed(
    mut db: Connection<Stargazers>,
    num: u32,
) -> std::result::Result<Json<Vec<(String, u32)>>, String> {
    match get_top_subscribed(&mut db, num).await {
        Ok(repos) => Ok(Json(repos)),
        Err(e) => Err(e.to_string()),
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[allow(dead_code)]
#[options("/<_..>")]
fn all_options() {}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
