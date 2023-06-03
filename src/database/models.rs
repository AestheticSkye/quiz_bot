use diesel::prelude::*;
use rand::random;

use crate::database::schema::*;

#[derive(Insertable)]
#[diesel(table_name = quizzes)]
pub struct NewQuiz {
    pub id: i32,
    pub owner_id: i64,
    pub title: String,
}

impl NewQuiz {
    pub fn new(owner_id: i64, title: &str) -> NewQuiz {
        NewQuiz {
            id: random(),
            owner_id,
            title: title.to_string(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion {
    pub quiz_id: i32,
    pub question: String,
}

#[derive(Insertable)]
#[diesel(table_name = answers)]
pub struct NewAnswer {
    pub question_id: i32,
    pub text: String,
    pub correct: bool,
}
