use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::{
    CommandResult,
    CommandError,
    macros::command,
    Args,
    Delimiter,
};

#[command]
#[only_in(guilds)]
#[num_args(1)]
pub fn purge(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    let amount = match args.advance().single::<u64>() {
        Ok(num) => num,
        Err(_) => { return Err(CommandError("Girdiğiniz sayı geri verildi.".into())); }
    };

    msg.channel_id.broadcast_typing(&ctx).ok();

    let with_id = amount > 100;
    let mut messages = msg.channel_id.messages(&ctx, |builder| {
        let builder = builder.before(msg.id);
        if with_id {
            builder.after(amount)
        } else {
            builder.limit(amount)
        }
    })?;

    if with_id {
        messages.remove(0);
    }

    msg.channel_id.delete_messages(&ctx, messages).map_err(|e| { CommandError::from(e) })
}
