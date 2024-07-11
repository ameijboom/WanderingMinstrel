use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct Opts {
    #[clap(long, env = "XIV_API_KEY")]
    pub xiv_api_key: String,

    #[clap(long, env = "DISCORD_TOKEN")]
    pub discord_token: String,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
