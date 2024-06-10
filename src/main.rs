mod commands;

use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage},
    async_trait,
    model::{
        application::{Command, Interaction},
        channel::Message,
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

use mafia;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // set handler for "message" event - called whenever a new message is received
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {:?}", command);

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "new" => Some(commands::new::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
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
            .set_commands(&ctx.http, vec![commands::ping::register()])
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
        | GatewayIntents::MESSAGE_CONTENT;

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
