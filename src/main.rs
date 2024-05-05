use std::path::PathBuf;

use analyse::analyse;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use download::download;

mod analyse;
mod db;
mod download;
mod github;

/// Fetch and analyse GitHub stargazers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    /// Path to database file
    #[arg(short, long, default_value = "stargazers.db")]
    db: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Download data from GitHub
    Download(DownloadArgs),

    /// Analyse data and produce data file for GUI
    Analyse(AnalyseArgs),
}

#[derive(Debug, Args)]
struct DownloadArgs {
    /// Owner name
    #[arg(short, long)]
    owner: String,

    /// Repository name
    #[arg(short, long)]
    repo: String,

    /// Personal access token
    #[arg(short, long)]
    pat: String,

    /// Only download new stargazers
    #[arg(short, long, default_value_t = false)]
    quick: bool,
}

#[derive(Debug, Args)]
struct AnalyseArgs {
    /// Path to data file
    #[arg(short, long, default_value = "data.json")]
    data: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let db = db::init(&cli.db)?;

    match cli.command {
        Command::Download(args) => {
            download(&db, &args.pat, &args.owner, &args.repo, args.quick).await?
        }
        Command::Analyse(args) => analyse(&db, &args.data).await?,
    }

    Ok(())
}
