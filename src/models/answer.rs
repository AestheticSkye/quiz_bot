use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, PartialEq, Debug)]
#[diesel(table_name = crate::database::schema::answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub correct: bool,
}
