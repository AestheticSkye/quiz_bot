use sea_orm_migration::prelude::*;

use crate::m20230603_120131_create_question_table::Question;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Answer::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(Answer::Id)
							.integer()
							.auto_increment()
							.not_null()
							.primary_key(),
					)
					.col(ColumnDef::new(Answer::Text).string().not_null())
					.col(ColumnDef::new(Answer::Correct).boolean().not_null())
					.col(ColumnDef::new(Answer::QuestionId).integer().not_null())
					.foreign_key(
						ForeignKey::create()
							.name("fk-answer-question_id")
							.from(Answer::Table, Answer::QuestionId)
							.to(Question::Table, Question::Id),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(Answer::Table).to_owned())
			.await
	}
}

#[derive(Iden)]
pub enum Answer {
	Table,
	Id,
	Text,
	Correct,
	QuestionId,
}
