use clap::Parser;
use anyhow::anyhow;
use poise::serenity_prelude as serenity;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct Opts {
    #[clap(long, env = "XIV_API_TOKEN")]
    pub xiv_api_token: String,

    #[clap(long, env = "DISCORD_TOKEN")]
    pub discord_token: String,

    #[clap(long, env = "DISCORD_CHANNEL")]
    pub discord_channel: String
}

pub struct Data {
    channel: serenity::ChannelId,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;
    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    channel: serenity::ChannelId::new(opts.discord_channel.parse()?),
                })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
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
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {}
        serenity::FullEvent::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available,
        } => {}
        _ => {}
    }

    Ok(())
}
