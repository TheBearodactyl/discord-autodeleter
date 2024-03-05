use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::macros::*;
use serenity::framework::standard::*;
use crate::Handler;

#[help]
pub async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: std::collections::HashSet<serenity::model::id::UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[command]
pub async fn setrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let role_id: RoleId = args.single()?;
    let mut data = ctx.data.write().await;
    let autodelete_roles = data.get_mut::<Handler>().unwrap();
    autodelete_roles.lock().await.insert(role_id);
    msg.channel_id
        .say(&ctx.http, format!("Autodelete role {} added.", role_id))
        .await?;
    Ok(())
}

#[command]
pub async fn getrole(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let autodelete_roles = data.get::<Handler>().unwrap();

    let mut role_names = Vec::new();
    for role_id in &*autodelete_roles.lock().await {
        if let Some(guild) = ctx.cache.guild(GuildId(917057579039989770)).await {
            if let Some(role) = guild.roles.get(role_id) {
                role_names.push(role.name.clone());
            }
        }
    }

    if !role_names.is_empty() {
        let role_list = role_names.join(", ");
        msg.channel_id
            .say(
                &ctx.http,
                format!("Current autodelete roles: {}", role_list),
            )
            .await?;
    } else {
        msg.channel_id
            .say(&ctx.http, "No autodelete roles set.")
            .await?;
    }

    Ok(())
}
