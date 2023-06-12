// TODO: Allow quizzes to be edited at any time

use poise::command;

use crate::commands::reply;
use crate::{Context, Error};

// TODO: Disallow multiple quizzes with the same name from the same author
/// Create a new quiz
#[command(prefix_command, slash_command)]
pub async fn new(ctx: Context<'_>, title: String) -> Result<(), Error> {
	let quiz_creation = ctx
		.data()
		.save_quiz_creation(*ctx.author().id.as_u64(), &title)
		.await?;

	reply(&ctx, &format!("Your quiz ID is {}.\nAdd questions by using `/quiz add question`.\nStart the quiz by using `/quiz start`", quiz_creation.id)).await?;

	Ok(())
}

#[command(prefix_command, slash_command, subcommands("question", "answer"))]
pub async fn add(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}

/// Add a question to the quiz you're currently editing
#[command(prefix_command, slash_command)]
pub async fn question(ctx: Context<'_>, text: String) -> Result<(), Error> {
	let Some(quiz_creation) = ctx
		.data()
		.fetch_quiz_creation(*ctx.author().id.as_u64())
		.await? else {
		reply(&ctx, "You have not created a quiz yet, create one by using `/quiz new`").await?;
		return Ok(())
	};

	ctx.data().add_question(&quiz_creation, &text).await?;

	reply(
		&ctx,
		&format!(
			"Question `{text}` added.\nAdd answers to this question by using `/quiz add answer`",
		),
	)
	.await?;

	Ok(())
}

/// And an answer to the last added question, at least one answer for each question has to be true
#[command(prefix_command, slash_command)]
pub async fn answer(ctx: Context<'_>, text: String, correct: bool) -> Result<(), Error> {
	let Some(quiz_creation) = ctx
		.data()
		.fetch_quiz_creation(*ctx.author().id.as_u64())
		.await? else {
		reply(&ctx, "You have not created a quiz yet, create one by using `/quiz new`").await?;
		return Ok(())
	};

	let Some(question) = ctx.data().fetch_current_creation_question(quiz_creation).await? else {
		reply(&ctx, "You have not added any questions yet, create one by using `/quiz add question").await?;
		return Ok(())
	};

	ctx.data().add_answer(&question, &text, correct).await?;

	reply(&ctx, &format!("Answer `{text}` added.\n")).await?;

	Ok(())
}
