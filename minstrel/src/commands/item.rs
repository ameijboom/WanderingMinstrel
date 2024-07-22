use crate::commands::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Item to look up by name"] item: String,
    #[description = "Type of item (if known)"] item_type: Option<String>,
) -> Result<(), Error> {
    let response = ctx.data().xivapi.search_item(&item).await?;
    let results = response
        .results
        .into_iter()
        .filter(|i| {
            item_type
                .as_ref()
                .map(|ty| &i.url_type == ty)
                .unwrap_or(true)
        })
        .map(|i| i.id.to_string())
        .collect::<Vec<_>>();

    ctx.say(results.join(", ")).await?;

    Ok(())
}
