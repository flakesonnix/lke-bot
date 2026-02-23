/// Roll a dice
#[poise::command(slash_command)]
pub async fn roll(
    ctx: crate::Context<'_>,
    #[description = "Number of sides on the dice"] sides: Option<u32>,
) -> Result<(), crate::Error> {
    use rand::RngExt;

    let sides = sides.unwrap_or(6);

    if sides < 2 {
        ctx.say("A dice must have at least 2 sides!").await?;
        return Ok(());
    }

    let result: u32 = rand::rng().random_range(1..=sides);

    ctx.say(format!("You rolled a {} on a {}-sided die!", result, sides))
        .await?;
    Ok(())
}
