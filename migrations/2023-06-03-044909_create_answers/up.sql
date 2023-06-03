-- Your SQL goes here
CREATE TABLE answers (
                         id SERIAL PRIMARY KEY,
                         question_id SERIAL REFERENCES questions(id) NOT NULL,
                         text VARCHAR NOT NULL,
                         correct BOOLEAN NOT NULL
)