use clap::Parser;
use joaju_unfurl::{fetch_and_unfurl, render_unfurl_typst_card};

#[derive(Parser, Debug)]
#[command(name = "joaju-unfurl")]
#[command(
    about = "A fast and secure Rust CLI for OpenGraph metadata extraction with SSRF protection and visual card rendering"
)]
struct Cli {
    /// URL to unfurl and extract metadata from
    url: String,

    /// Output Typst visual card format instead of JSON
    #[arg(short, long)]
    typst: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let meta = fetch_and_unfurl(&cli.url).await?;

    if cli.typst {
        println!("{}", render_unfurl_typst_card(&meta, &cli.url));
    } else {
        println!("{}", serde_json::to_string_pretty(&meta)?);
    }

    Ok(())
}
