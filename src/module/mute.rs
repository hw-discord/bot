use crate::muteable::Muteable;

use chrono::Duration;
use futures::StreamExt;
use serenity::{
    client::Context, framework::standard::macros::command, framework::standard::Args,
    framework::standard::CommandError, framework::standard::CommandResult, model::channel::Message,
    model::guild::Member,
};

#[command]
#[max_args(3)]
pub async fn mute(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    find_member(ctx, msg, &args.single::<String>()?)
        .await?
        .mute(
            ctx.http.clone(),
            args.single::<i64>().ok().map(Duration::minutes),
            args.remains(),
        )
        .await
        .map_err(CommandError::from)
}

#[command]
#[max_args(2)]
pub async fn unmute(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    find_member(ctx, msg, &args.single::<String>()?)
        .await?
        .unmute(
            ctx.http.clone(),
            args.single::<i64>().ok().map(Duration::minutes),
        )
        .await
        .map_err(CommandError::from)
}

pub async fn find_member(ctx: &Context, msg: &Message, name: &str) -> serenity::Result<Member> {
    fn contains(substring: &str, name: &str) -> bool {
        name.to_lowercase().contains(&substring.to_lowercase())
    }

    let members = msg
        .guild_id
        .ok_or(serenity::Error::Other("guild!!?"))?
        .members_iter(ctx)
        .filter_map(|member| async move {
            if member.is_err() {
                return None;
            }

            let member = member.unwrap();

            let username = &member.user.name;
            if contains(name, username) {
                Some(member)
            } else {
                member.nick.clone().and_then(|nick| {
                    if contains(name, &nick) {
                        Some(member)
                    } else {
                        None
                    }
                })
            }
        })
        .collect::<Vec<Member>>()
        .await;

    Ok(members
        .first()
        .ok_or(serenity::Error::Other("kim bu amk tanımıyorum."))?
        .clone())
}
