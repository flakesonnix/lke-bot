/// Ping the bot and check latency
#[poise::command(slash_command)]
pub async fn ping(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let latency = ctx.ping().await.as_millis();
    ctx.say(format!("Pong! Latency: {}ms", latency)).await?;
    Ok(())
}
