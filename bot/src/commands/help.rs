/// Show available commands
#[poise::command(slash_command)]
pub async fn help(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let reply = "**Available Commands**\n\
    `/ping` - Check bot latency\n\
    `/userinfo [user]` - Display information about a user\n\
    `/help` - Show this help message\n\
    `/roll [sides]` - Roll a dice (default 6 sides)";

    ctx.say(reply).await?;
    Ok(())
}
