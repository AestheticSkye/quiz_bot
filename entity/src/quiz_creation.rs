//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "quiz_creation")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id:                  Uuid,
	pub owner_id:            i64,
	pub current_question_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::question::Entity",
		from = "Column::CurrentQuestionId",
		to = "super::question::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Question,
	#[sea_orm(
		belongs_to = "super::quiz::Entity",
		from = "Column::Id",
		to = "super::quiz::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Quiz,
}

impl Related<super::question::Entity> for Entity {
	fn to() -> RelationDef { Relation::Question.def() }
}

impl Related<super::quiz::Entity> for Entity {
	fn to() -> RelationDef { Relation::Quiz.def() }
}

impl ActiveModelBehavior for ActiveModel {}
