// mod commands;
// mod database;
// mod models;

mod database;
mod models;

use dotenvy::dotenv;
use poise::serenity_prelude::GatewayIntents;
use tracing::info;

use poise::command;

// use crate::commands::basic_commands::*;
// use crate::commands::character_commands::*;
use crate::database::Database;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Database, Error>;

/// Hello World!
#[command(prefix_command, slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("World!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to find .env file");
    tracing_subscriber::fmt::init();

    let options = poise::FrameworkOptions {
        commands: vec![hello()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("$".into()),
            // additional_prefixes: vec![poise::Prefix::Regex(
            //     regex::Regex::new("(?i)(elexis)(?i),?").unwrap(),
            // )],
            ..Default::default()
        },
        /// This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        /// This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        on_error: |err| {
            Box::pin(async move {
                println!(
                    "Failed to execute command {:?}!",
                    err.ctx().map(|ctx| ctx.command().qualified_name.clone())
                );
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Database::new())
            })
        });

    framework.run().await.unwrap();
}
