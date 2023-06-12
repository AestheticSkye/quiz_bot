// Due to empty commands with subcommands
#![allow(clippy::unused_async)]

mod creation_commands;

use std::time::Duration;

use poise::command;
use tokio::time::sleep;
use tracing::info;
use uuid::Uuid;

use crate::commands::creation_commands::{add, new};
use crate::{Context, Error};

#[command(
	prefix_command,
	slash_command,
	subcommands("new", "add", "run", "list", "end")
)]
pub async fn quiz(_: Context<'_>) -> Result<(), Error> {
	Ok(())
}

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

// TODO: Run off of quiz name rather than id
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

	ctx.say(format!(
		"Running quiz `{}`\nYou will have 30 seconds to answer each question",
		quiz.text
	))
	.await?;

	ctx.data()
		.add_active_quiz(*ctx.author().id.as_u64(), *ctx.channel_id().as_u64())
		.await;

	let questions = ctx.data().fetch_questions(&quiz).await?;

	for question in questions {
		if ctx
			.data()
			.fetch_active_quiz(*ctx.channel_id().as_u64())
			.await
			.is_none()
		{
			// FIXME: This doesn't get printed yet
			ctx.say("Quiz has ended").await?;
		}

		let answers = ctx.data().fetch_answers(&question).await?;

		let mut reply = format!("**Question: {}\n**", question.text);

		answers.iter().enumerate().for_each(|(index, answer)| {
			reply += format!("{}. {}\n", index + 1, answer.text).as_str();
		});

		ctx.say(reply).await?;

		sleep(Duration::new(10, 0)).await;
	}

	Ok(())
}

/// End the quiz that is currently running.
/// Can only be done by the user who started the quiz
#[command(prefix_command, slash_command)]
async fn end(ctx: Context<'_>) -> Result<(), Error> {
	let channel_id = *ctx.channel_id().as_u64();

	let Some(active_quiz) = ctx.data().fetch_active_quiz(channel_id).await else {
		reply(&ctx, "There is no quiz active in this channel").await?;
		return Ok(())
	};

	if active_quiz.owner_id != *ctx.author().id.as_u64() {
		reply(&ctx, "You do not have permission to end this quiz").await?;
		return Ok(());
	}

	info!("The quiz will end when this question is finished");
	ctx.say("The quiz will end when this question is finished")
		.await?;

	ctx.data().remove_active_quiz(channel_id).await;

	Ok(())
}

pub async fn reply(ctx: &Context<'_>, text: &str) -> Result<(), Error> {
	ctx.send(|b| b.content(text).ephemeral(true).reply(true))
		.await?;

	Ok(())
}
