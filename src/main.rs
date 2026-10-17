use clap::Parser;
use unfurl_rs::{fetch_and_unfurl, render_unfurl_typst_card};

#[derive(Parser, Debug)]
#[command(name = "unfurl-rs")]
#[command(
    about = "Motor soberano em Rust para extração de metadados OpenGraph com blindagem SSRF e cartões visuais"
)]
struct Cli {
    /// URL para extrair metadados e gerar cartão
    url: String,

    /// Emitir formato de cartão Typst em vez de JSON
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
