-- Your SQL goes here
CREATE TABLE quizzes (
                         id INT PRIMARY KEY,
                         owner_id BIGINT NOT NULL,
                         title VARCHAR NOT NULL
)