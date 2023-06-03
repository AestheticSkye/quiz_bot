-- Your SQL goes here
CREATE TABLE questions (
                           id INT PRIMARY KEY,
                           quiz_id INT REFERENCES quizzes(id) NOT NULL,
                           question VARCHAR NOT NULL
)