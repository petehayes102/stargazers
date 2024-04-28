use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, AUTHORIZATION},
    Client,
};

mod types;
pub use self::types::*;

const GITHUB_URL: &str = "https://api.github.com";
const USER_AGENT: &str = "Stargazer Gazer";

pub fn create_client(pat: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();
    let mut auth: HeaderValue = format!("Bearer {pat}").parse()?;
    auth.set_sensitive(true);
    headers.insert(AUTHORIZATION, auth);
    headers.insert(ACCEPT_ENCODING, "application/json".parse()?);
    headers.insert(ACCEPT, "application/vnd.github.star+json".parse()?);

    Ok(Client::builder()
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?)
}

pub async fn get_repository(client: &Client, owner: &str, repo: &str) -> Result<Repository> {
    let url = format!("{GITHUB_URL}/repos/{owner}/{repo}");
    let reply = client.get(&url).send().await?;
    Ok(reply.error_for_status()?.json().await?)
}

// pub async fn get_stargazers(client: &Client, owner: &str, repo: &str) -> Result<Vec<User>> {
//     let url = format!("{GITHUB_URL}/repos/{owner}/{repo}/stargazers");
//     let mut users = Vec::new();

//     let mut looping = true;
//     let mut page = 1;
//     while looping {
//         let reply = client
//             .get(&url)
//             .query(&[("page", page), ("per_page", 100)])
//             .send()
//             .await?;

//         let mut u: Vec<User> = reply.error_for_status()?.json().await?;

//         if u.len() < 100 {
//             looping = false;
//         }

//         users.append(&mut u);
//         page += 1;
//     }

//     Ok(users)
// }
