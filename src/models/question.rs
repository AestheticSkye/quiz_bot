use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, PartialEq, Debug)]
#[diesel(table_name = crate::database::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Question {
	pub id:       i32,
	pub quiz_id:  i32,
	pub question: String,
}
