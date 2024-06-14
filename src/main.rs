use ::serenity::all::{Mentionable, UserId, VoiceState};
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};

use std::collections::HashMap;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Create a new game of Mafia with the people in your VC.
#[poise::command(slash_command, prefix_command)]
async fn new(ctx: Context<'_>) -> Result<(), Error> {
    // let cache = ctx.cache();
    let user = ctx.author();
    // let guild = ctx.guild().unwrap();

    // let mut user_in_vc = false;
    // let mut players = Vec::new();

    // let voice_states: &HashMap<UserId, VoiceState> = &guild.voice_states;

    // // println!("{:#?}", voice_states);

    // // if voice_states.is_empty() {
    // //     return Err("User not in a voice channel".into());
    // // }
    // let user_vc;
    // if let Some(channel_id) = voice_states[&(user.id)].channel_id {
    //     user_vc = channel_id;
    // } else {
    //     return Err("User not in a voice channel".into());
    // }

    // for (user_id, voice_state) in voice_states {
    //     if let Some(c_id) = voice_state.channel_id {
    //         if c_id == user_vc {
    //             players.push(user_id);
    //             // user_in_vc = true;
    //         }
    //     }
    // }

    let players: Vec<UserId> = {
        let Some(guild) = ctx.guild() else {
            return Err("No guild".into());
        };

        let voice_states: &HashMap<UserId, VoiceState> = &guild.voice_states;
        let user_vc;
        if let Some(channel_id) = voice_states[&(user.id)].channel_id {
            user_vc = channel_id;
        } else {
            return Err("User not in a voice channel".into());
        }

        guild
            .voice_states
            .iter()
            .filter(|(_, state)| state.channel_id.is_some_and(|c_id| c_id == user_vc))
            .map(|(user_id, _)| *user_id)
            .collect()
    };

    println!("{:?}", players);

    let embed = serenity::CreateEmbed::new().title("Players:").fields(vec![(
        "test",
        &user.id.mention().to_string(),
        true,
    )]);

    ctx.send(poise::CreateReply::default().embed(embed).reply(false))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = include_str!("BotToken.txt")
        .split_whitespace()
        .next()
        .unwrap();
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_VOICE_STATES
        | serenity::GatewayIntents::GUILDS;
    let prefix_options = PrefixFrameworkOptions {
        prefix: Some("!!".into()),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), new()],
            prefix_options,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
