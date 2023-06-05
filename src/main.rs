mod commands;
mod database;

use dotenvy::dotenv;
pub use entity::answer::Model as Answer;
pub use entity::question::Model as Question;
pub use entity::quiz::Model as Quiz;
pub use entity::quiz_creation::Model as QuizCreation;
use poise::command;
use poise::serenity_prelude::GatewayIntents;
use tracing::info;

use crate::commands::quiz;
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
	tracing_subscriber::fmt()
		.with_test_writer()
		// .with_max_level(tracing::Level::DEBUG)
		.init();

	let options = poise::FrameworkOptions {
		commands: vec![hello(), quiz()],
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
				Ok(Database::new().await.unwrap())
			})
		});

	framework.run().await.unwrap();
}
