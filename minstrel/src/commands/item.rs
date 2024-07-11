use poise::serenity_prelude as serenity;

use crate::{commands::{Context, Error}, xivapi};

#[poise::command(prefix_command, slash_command)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Item to look up by name"]
    item: String,
    #[description = "Type of item (if known)"]
    item_type: Option<String>
) -> Result<(), Error> {
    let mut results = ctx.data().xivapi.search_item(&item).await?;

    if item_type.is_some() {
        let t = item_type.unwrap();
        results.results.retain(|i| *i.url_type != t)
    }

    ctx.say(results.results.iter().map(|i| i.id.to_string()).collect::<String>()).await?;

    Ok(())
}
