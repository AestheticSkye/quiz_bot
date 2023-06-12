use std::env;

use entity::{answer, prelude, question, quiz, quiz_creation};
use migration::{Migrator, MigratorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::{
	ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter,
};
use tokio::sync::Mutex;
use tracing::info;
use uuid::Uuid;

use crate::{Answer, Error, Question, Quiz, QuizCreation};

#[derive(Clone)]
pub struct ActiveQuiz {
	pub owner_id: u64,
	pub channel_id: u64,
}

pub struct Database {
	connection: DatabaseConnection,
	active_quizzes: Mutex<Vec<ActiveQuiz>>,
}

impl Database {
	pub async fn new() -> Self {
		let database_url = env::var("DATABASE_URL").expect("Could not find database URL");

		let connection: DatabaseConnection = sea_orm::Database::connect(database_url.clone())
			.await
			.expect("Could not connect to database");

		Migrator::up(&connection, None)
			.await
			.expect("Could not migrate database");

		info!("Connected to database at {}", database_url);

		Self {
			connection,
			active_quizzes: Mutex::new(vec![]),
		}
	}

	pub async fn save_quiz_creation(
		&self,
		owner_id: u64,
		text: &str,
	) -> Result<QuizCreation, Error> {
		let quiz = self.save_quiz(owner_id, text).await?;

		let quiz_creation = quiz_creation::ActiveModel {
			id: Set(quiz.id),
			owner_id: Set(owner_id.try_into()?),
			current_question_id: NotSet,
		};

		Ok(quiz_creation.insert(&self.connection).await?)
	}

	pub async fn fetch_quiz(&self, quiz_id: &Uuid) -> Result<Option<Quiz>, Error> {
		Ok(prelude::Quiz::find_by_id(*quiz_id)
			.one(&self.connection)
			.await?)
	}

	pub async fn fetch_all_quizzes(&self, owner_id: u64) -> Result<Vec<Quiz>, Error> {
		let owner_id: i64 = owner_id.try_into()?;

		Ok(prelude::Quiz::find()
			.filter(quiz::Column::OwnerId.eq(owner_id))
			.all(&self.connection)
			.await?)
	}

	pub async fn fetch_questions(&self, quiz: &Quiz) -> Result<Vec<Question>, Error> {
		Ok(prelude::Question::find()
			.filter(question::Column::QuizId.eq(quiz.id))
			.all(&self.connection)
			.await?)
	}

	pub async fn fetch_answers(&self, question: &Question) -> Result<Vec<Answer>, Error> {
		Ok(prelude::Answer::find()
			.filter(answer::Column::QuestionId.eq(question.id))
			.all(&self.connection)
			.await?)
	}

	pub async fn fetch_quiz_creation(&self, owner_id: u64) -> Result<Option<QuizCreation>, Error> {
		Ok(prelude::QuizCreation::find()
			.filter(quiz_creation::Column::OwnerId.eq(owner_id))
			.one(&self.connection)
			.await?)
	}

	pub async fn fetch_current_creation_question(
		&self,
		quiz_creation: QuizCreation,
	) -> Result<Option<Question>, Error> {
		let Some(current_question_id) = quiz_creation.current_question_id else {
			return Ok(None)
		};

		Ok(prelude::Question::find_by_id(current_question_id)
			.one(&self.connection)
			.await?)
	}

	pub async fn add_question(
		&self,
		quiz_creation: &QuizCreation,
		question: &str,
	) -> Result<QuizCreation, Error> {
		let question = question::ActiveModel {
			id: NotSet,
			text: Set(question.to_owned()),
			quiz_id: Set(quiz_creation.id),
		};

		let question = question.insert(&self.connection).await?;

		let mut quiz_creation: quiz_creation::ActiveModel = quiz_creation.clone().into();

		quiz_creation.current_question_id = Set(Some(question.id));

		Ok(quiz_creation.update(&self.connection).await?)
	}

	pub async fn add_answer(
		&self,
		question: &Question,
		text: &str,
		correct: bool,
	) -> Result<(), Error> {
		let answer = answer::ActiveModel {
			id: NotSet,
			text: Set(text.to_owned()),
			correct: Set(correct),
			question_id: Set(question.id),
		};

		answer.insert(&self.connection).await?;
		Ok(())
	}

	pub async fn add_active_quiz(&self, owner_id: u64, channel_id: u64) {
		self.active_quizzes.lock().await.push(ActiveQuiz {
			owner_id,
			channel_id,
		});
	}

	pub async fn remove_active_quiz(&self, channel_id: u64) {
		if let Some(index) = self
			.active_quizzes
			.lock()
			.await
			.iter()
			.position(|active_quiz| active_quiz.channel_id == channel_id)
		{
			self.active_quizzes.lock().await.remove(index);
		}
	}

	pub async fn fetch_active_quiz(&self, channel_id: u64) -> Option<ActiveQuiz> {
		let Some(index) = self.active_quizzes.lock()
			.await.iter().position(|active_quiz| active_quiz.channel_id == channel_id) else {
			return None
		};

		Some(self.active_quizzes.lock().await[index].clone())
	}

	async fn save_quiz(&self, owner_id: u64, text: &str) -> Result<Quiz, Error> {
		let quiz = quiz::ActiveModel {
			id: Set(Uuid::new_v4()),
			owner_id: Set(owner_id.try_into()?),
			text: Set(text.to_owned()),
		};

		Ok(quiz
			.insert(&self.connection)
			.await
			.expect("Failed to insert quiz"))
	}
}

#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn test() {
	use dotenvy::dotenv;

	dotenv().unwrap();

	let db = Database::new().await;

	// db.save_quiz_creation(42, "Test").await;

	let quiz_creation = db.fetch_quiz_creation(42).await.unwrap().unwrap();

	// quiz_creation = db.add_question(quiz_creation.clone(), "A Question").await;

	let question = db
		.fetch_current_creation_question(quiz_creation)
		.await
		.unwrap()
		.unwrap();

	db.add_answer(&question, "straight", false).await.unwrap();
}
