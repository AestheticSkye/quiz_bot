use sea_orm_migration::prelude::*;

use crate::m20230603_113226_create_quiz_table::Quiz;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Question::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(Question::Id)
							.integer()
							.auto_increment()
							.not_null()
							.primary_key(),
					)
					.col(ColumnDef::new(Question::Text).string().not_null())
					.col(ColumnDef::new(Question::QuizId).uuid().not_null())
					.foreign_key(
						ForeignKey::create()
							.name("fk-question-quiz_id")
							.from(Question::Table, Question::QuizId)
							.to(Quiz::Table, Quiz::Id),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(Question::Table).to_owned())
			.await
	}
}

#[derive(Iden)]
pub enum Question {
	Table,
	Id,
	Text,
	QuizId,
}
