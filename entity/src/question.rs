//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "question")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id:      i32,
	pub text:    String,
	pub quiz_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::answer::Entity")]
	Answer,
	#[sea_orm(
		belongs_to = "super::quiz::Entity",
		from = "Column::QuizId",
		to = "super::quiz::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Quiz,
	#[sea_orm(has_many = "super::quiz_creation::Entity")]
	QuizCreation,
}

impl Related<super::answer::Entity> for Entity {
	fn to() -> RelationDef { Relation::Answer.def() }
}

impl Related<super::quiz::Entity> for Entity {
	fn to() -> RelationDef { Relation::Quiz.def() }
}

impl Related<super::quiz_creation::Entity> for Entity {
	fn to() -> RelationDef { Relation::QuizCreation.def() }
}

impl ActiveModelBehavior for ActiveModel {}