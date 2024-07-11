use clap::Parser;
use anyhow::anyhow;
use poise::serenity_prelude as serenity;
use tracing_subscriber::EnvFilter;

mod commands;
mod xivapi;

#[derive(Parser)]
struct Opts {
    #[clap(long, env = "XIV_API_TOKEN")]
    pub xiv_api_token: String,

    #[clap(long, env = "DISCORD_TOKEN")]
    pub discord_token: String,

    #[clap(long, env = "DISCORD_CHANNEL")]
    pub discord_channel: String
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let options = poise::FrameworkOptions {
        commands: vec![commands::item::search()],
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };
    
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(commands::Data {
                    xivapi: xivapi::client::Client::new(opts.xiv_api_token)
                })
            })
        })
        .options(options)
        .build();

    let mut client = serenity::ClientBuilder::new(opts.discord_token, intents)
        .framework(framework)
        .await?;

    client.start().await.map_err(|e| {
        tracing::error!({error = ?e}, "error starting client");
        anyhow!(e)
    })
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, commands::Data, commands::Error>,
    data: &commands::Data,
) -> Result<(), commands::Error> {
    match event {
        _ => {}
    }

    Ok(())
}
