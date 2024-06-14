use serenity::{
    all::{ChannelType, Context, GuildId},
    cache::Cache,
    model::{
        application::CommandInteraction,
        guild::{self, Guild, Member},
        user::User,
    },
    prelude::*,
};

pub struct Mafia {
    players: Vec<User>,
    points: Vec<u64>,
}

impl Mafia {
    pub async fn new(ctx: Context, command: CommandInteraction) -> Result<Mafia, String> {
        let user = command.user;
        let guild_id = command.guild_id.unwrap();
        let mut user_in_vc = false;
        let mut players = Vec::new();
        for channel in guild_id.channels(&ctx.http).await.unwrap().values() {
            if channel.kind == ChannelType::Voice {
                let members: Vec<User> = channel
                    .members(ctx.cache.clone())
                    .unwrap()
                    .iter()
                    .map(|m| m.user.clone())
                    .collect();
                if !members.is_empty() && members.contains(&user) {
                    user_in_vc = true;
                    players = members.clone();
                }
            }
        }
        let size = players.len();

        if user_in_vc {
            Ok(Mafia {
                players,
                points: vec![0; size],
            })
        } else {
            Err("User {user} not in a voice channel.".to_string())
        }
    }
}
