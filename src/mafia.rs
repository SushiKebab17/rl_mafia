use serenity::{
    all::{ChannelType, Context},
    model::{channel::Message, guild::Guild},
    prelude::*,
};

pub struct Mafia {
    players: Vec,
    points: Vec<u64>,
}

impl Mafia {
    pub fn new(guild: Guild, ctx: Context, msg: Message) -> Mafia {
        let mut user_in_vc = false;
        for channel in guild.channels.values() {
            if channel.kind == ChannelType::Voice {
                let members = channel.members(cache)
            }
        }

        Mafia {}
    }
}
