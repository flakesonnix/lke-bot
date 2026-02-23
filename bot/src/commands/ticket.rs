/// Create a support ticket
#[poise::command(slash_command, guild_only)]
pub async fn ticket(
    ctx: crate::Context<'_>,
    #[description = "Title/subject of the ticket"] title: String,
) -> Result<(), crate::Error> {
    let _guild_id = ctx.guild_id().map(|g| g.get()).unwrap_or(0);
    
    ctx.say(format!("Creating ticket: {}...", title)).await?;
    
    Ok(())
}

/// Close the current ticket
#[poise::command(slash_command, guild_only)]
pub async fn close(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    ctx.say("Closing ticket...").await?;
    Ok(())
}
