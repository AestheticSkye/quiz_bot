use sea_orm_migration::prelude::*;

use crate::m20230603_113226_create_quiz_table::Quiz;
use crate::m20230603_120131_create_question_table::Question;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(QuizCreation::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(QuizCreation::Id)
							.uuid()
							.not_null()
							.primary_key(),
					)
					.col(
						ColumnDef::new(QuizCreation::OwnerId)
							.big_unsigned()
							.not_null(),
					)
					.col(ColumnDef::new(QuizCreation::CurrentQuestionId).integer())
					.foreign_key(
						ForeignKey::create()
							.name("fk-quiz_creation-quiz_id")
							.from(QuizCreation::Table, QuizCreation::Id)
							.to(Quiz::Table, Quiz::Id),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk-current_question-question_id")
							.from(QuizCreation::Table, QuizCreation::CurrentQuestionId)
							.to(Question::Table, Question::Id),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		// Replace the sample below with your own migration scripts
		manager
			.drop_table(Table::drop().table(QuizCreation::Table).to_owned())
			.await
	}
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum QuizCreation {
	Table,
	Id,
	OwnerId,
	CurrentQuestionId,
}
