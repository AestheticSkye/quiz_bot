FROM rust:1.69

WORKDIR /usr/src/quiz_bot
COPY . .

RUN cargo install --path .

CMD ["quiz_bot"]