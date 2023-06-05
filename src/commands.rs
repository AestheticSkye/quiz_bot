use std::time::Duration;

use poise::command;
use tokio::time::sleep;
use uuid::Uuid;

use crate::{Context, Error};

#[command(prefix_command, slash_command, subcommands("new", "add", "run"))]
pub async fn quiz(_: Context<'_>) -> Result<(), Error> { Ok(()) }

#[command(prefix_command, slash_command)]
pub async fn new(ctx: Context<'_>, title: String) -> Result<(), Error> {
	let quiz_creation = ctx
		.data()
		.save_quiz_creation(*ctx.author().id.as_u64(), &title)
		.await;

	reply(&ctx, &format!("Your quiz ID is {}.\nAdd questions by using `/quiz add question`.\nStart the quiz by using `/quiz start`", quiz_creation.id)).await?;

	Ok(())
}

#[command(prefix_command, slash_command, subcommands("question", "answer"))]
pub async fn add(_ctx: Context<'_>) -> Result<(), Error> { Ok(()) }

#[command(prefix_command, slash_command)]
pub async fn question(ctx: Context<'_>, text: String) -> Result<(), Error> {
	let Some(quiz_creation) = ctx
		.data()
		.fetch_quiz_creation(*ctx.author().id.as_u64())
		.await else {
		reply(&ctx, "You have not created a quiz yet, create one by using `/quiz new`").await?;
		return Ok(())
	};

	ctx.data().add_question(quiz_creation, &text).await;

	reply(
		&ctx,
		&format!(
			"Question `{}` added.\nAdd answers to this question by using `/quiz add answer`",
			text
		),
	)
	.await?;

	Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn answer(ctx: Context<'_>, text: String, correct: bool) -> Result<(), Error> {
	let Some(quiz_creation) = ctx
		.data()
		.fetch_quiz_creation(*ctx.author().id.as_u64())
		.await else {
		reply(&ctx, "You have not created a quiz yet, create one by using `/quiz new`").await?;
		return Ok(())
	};

	let Some(question) = ctx.data().fetch_current_creation_question(quiz_creation).await else {
		reply(&ctx, "You have not added any questions yet, create one by using `/quiz add question").await?;
		return Ok(())
	};

	ctx.data().add_answer(question, &text, correct).await;

	reply(&ctx, &format!("Answer `{}` added.\n", text)).await?;

	Ok(())
}

#[command(prefix_command, slash_command)]
pub async fn run(ctx: Context<'_>, quiz_id: String) -> Result<(), Error> {
	let Ok(quiz_id) = Uuid::try_parse(&quiz_id) else {
		ctx.say("Invalid quiz ID.").await?;
		return Ok(())
	};

	let Some(quiz) = ctx.data().fetch_quiz(&quiz_id).await else {
		ctx.say("Cannot find quiz under that id.").await?;
		return Ok(())
	};

	let questions = ctx.data().fetch_questions(&quiz).await;

	for question in questions {
		let answers = ctx.data().fetch_answers(&question).await;

		let mut reply = format!("**Question: {}\n**", question.text);

		answers.iter().enumerate().for_each(|(index, answer)| {
			reply += format!("{}. {}\n", index + 1, answer.text).as_str()
		});

		ctx.say(reply).await?;

		sleep(Duration::new(2, 0)).await;
	}

	Ok(())
}

async fn reply(ctx: &Context<'_>, text: &str) -> Result<(), Error> {
	ctx.send(|b| b.content(text).ephemeral(true).reply(true))
		.await?;

	Ok(())
}
