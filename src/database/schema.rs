// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        text -> Varchar,
        correct -> Bool,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        quiz_id -> Int4,
        question -> Varchar,
    }
}

diesel::table! {
    quizzes (id) {
        id -> Int4,
        owner_id -> Int8,
        title -> Varchar,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(questions -> quizzes (quiz_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
    quizzes,
);
