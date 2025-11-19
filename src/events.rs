mod message;

use serenity::all::Ready;
use serenity::prelude::*;
use serenity::{async_trait, model::channel::Message};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        message::on_message_create(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Online in {} guild(s)!", ctx.cache.guilds().len());
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<serenity::all::GuildId>) {
        for guild_id in guilds {
            if let Some(guild) = guild_id.to_guild_cached(&ctx.cache) {
                let name = guild.name.clone();
                let members = guild.member_count;

                println!("- {name}\n  members:{members}");
            }
        }
    }

    async fn guild_create(&self, _ctx: Context, guild: serenity::all::Guild, is_new: Option<bool>) {
        if is_new == Some(true) {
            info!(
                name: "guild_create",
                "Added to new guild <{}>", guild.name
            );
        }
    }
}
