// Due to empty commands with subcommands
#![allow(clippy::unused_async)]

mod creation_commands;

use std::time::Duration;

use poise::command;
use tokio::time::sleep;
use uuid::Uuid;

use crate::commands::creation_commands::{add, new};
use crate::{Context, Error};

#[command(
	prefix_command,
	slash_command,
	subcommands("new", "add", "run", "list")
)]
pub async fn quiz(_: Context<'_>) -> Result<(), Error> { Ok(()) }

#[command(prefix_command, slash_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
	let quizzes = ctx
		.data()
		.fetch_all_quizzes(*ctx.author().id.as_u64())
		.await?;

	if quizzes.is_empty() {
		reply(&ctx, "You have no quizzes yet").await?;
		return Ok(());
	}

	let response: String = quizzes
		.into_iter()
		.map(|quiz| format!("`{}` {}\n", quiz.id, quiz.text))
		.collect();

	reply(&ctx, &response).await?;

	Ok(())
}

/// Run a quiz
#[command(prefix_command, slash_command)]
async fn run(ctx: Context<'_>, quiz_id: String) -> Result<(), Error> {
	let Ok(quiz_id) = Uuid::try_parse(&quiz_id) else {
		ctx.say("Invalid quiz ID.").await?;
		return Ok(())
	};

	let Some(quiz) = ctx.data().fetch_quiz(&quiz_id).await? else {
		ctx.say("Cannot find quiz under that id.").await?;
		return Ok(())
	};

	let questions = ctx.data().fetch_questions(&quiz).await?;

	for question in questions {
		let answers = ctx.data().fetch_answers(&question).await?;

		let mut reply = format!("**Question: {}\n**", question.text);

		answers.iter().enumerate().for_each(|(index, answer)| {
			reply += format!("{}. {}\n", index + 1, answer.text).as_str();
		});

		ctx.say(reply).await?;

		sleep(Duration::new(2, 0)).await;
	}

	Ok(())
}

pub async fn reply(ctx: &Context<'_>, text: &str) -> Result<(), Error> {
	ctx.send(|b| b.content(text).ephemeral(true).reply(true))
		.await?;

	Ok(())
}
