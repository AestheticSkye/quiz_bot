use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, PartialEq, Debug)]
#[diesel(table_name = crate::database::schema::quizzes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quiz {
    pub id: i32,
    pub owner_id: i64,
    pub title: String,
}
