use serenity::client::Context;
use serenity::model::channel::{ Message, Reaction };
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;
use crate::module::{ badword, blacklink, presence, selfmod, slowmode, startup_message };

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, new_message: Message) {
        blacklink::message(&ctx, &new_message);
        badword::message(&ctx, &new_message);
        slowmode::message(&ctx, &new_message);
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        selfmod::reaction_add(&ctx, &reaction);
    }

    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        presence::ready(&ctx);
        startup_message::ready(&ctx);
    }
}
