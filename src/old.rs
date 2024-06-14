mod commands;

use rl_mafia::mafia;
use serenity::{
    all::{ChannelType, CreateInteractionResponse, CreateInteractionResponseMessage, User},
    async_trait,
    model::{
        application::{Command, Interaction},
        channel::Message,
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

// use mafia;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // set handler for "message" event - called whenever a new message is received
    async fn message(&self, ctx: Context, msg: Message) {
        let http = &ctx.http;
        let cache = &ctx.cache;

        if msg.content == "!ping" {
            // if let Err(why) = msg.channel_id.say(http, "Pong!").await {
            //     println!("Error sending message: {:?}", why);
            // }

            let user = msg.author;
            let guild_id = msg.guild_id.unwrap();

            let mut user_in_vc = false;
            let mut players = Vec::new();
            for channel in guild_id.channels(http).await.unwrap().values() {
                if channel.kind == ChannelType::Voice {
                    // println!("{}", channel.name);
                    // println!("{:#?}", cache);
                    println!("{:?}", cache.guilds());
                    let members: Vec<User> = channel
                        .members(cache)
                        .unwrap()
                        .iter()
                        .map(|m| m.user.clone())
                        .collect();

                    if members.contains(&user) {
                        user_in_vc = true;
                        players = members.clone();
                    }
                }
            }
            println!("{}\n{:?}", user_in_vc, players);

            // let voice_states = guild_id.voice_states(&ctx.http).await.unwrap_or_default();
            // let user = msg.author;
            // let mut user_in_vc = false;
            // let mut players = Vec::new();
            // for (user_id, voice_state) in voice_states {
            //     // Check if the user is in a voice channel
            //     if let Some(channel_id) = voice_state.channel_id {
            //         if user_id == user.id {
            //             user_in_vc = true;
            //         }
            //         // Fetch the user's information
            //         if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
            //             players.push(member.user);
            //         }
            //     }
            // }
            // // Inform the user if they are in a voice channel
            // let response = if user_in_vc {
            //     format!(
            //         "You are in a voice channel with: {:?}",
            //         players.iter().map(|u| &u.name).collect::<Vec<_>>()
            //     )
            // } else {
            //     "You are not in a voice channel.".to_string()
            // };
            // if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            //     println!("Error sending message: {:?}", why);
            // }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {:?}", command);

            if command.data.name.as_str() == "new" {
                let test = mafia::Mafia::new(ctx, command);
            }

            // let content = match command.data.name.as_str() {
            //     "ping" => Some(commands::ping::run(&command.data.options())),
            //     "new" => Some(commands::new::run(&command.data.options())),
            //     _ => Some("not implemented :(".to_string()),
            // };

            // if let Some(content) = content {
            //     let data = CreateInteractionResponseMessage::new().content(content);
            //     let builder = CreateInteractionResponse::Message(data);
            //     if let Err(why) = command.create_response(&ctx.http, builder).await {
            //         println!("Cannot respond to slash command: {}", why);
            //     }
            // }
        }
    }

    // set handler for the "ready" event - in this case, just prints out the current user's username.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let server = include_str!("GuildToken.txt")
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let guild_id = GuildId::new(server);

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![commands::ping::register(), commands::new::register()],
            )
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        // let guild_command =
        //     Command::create_global_command(&ctx.http, commands::wonderful_command::register())
        //         .await;

        // println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS;

    let token = include_str!("BotToken.txt")
        .split_whitespace()
        .next()
        .unwrap();
    // let id = ids.next().unwrap().parse().unwrap();

    // creates instance of Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // starts a single shard which listens to events.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
