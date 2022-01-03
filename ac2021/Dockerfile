FROM rust:1.56.1-slim

ENV APP_HOME /app
RUN mkdir $APP_HOME
WORKDIR $APP_HOME

COPY data $APP_HOME/data
COPY src $APP_HOME/src
COPY Cargo.toml $APP_HOME
COPY Cargo.lock $APP_HOME

RUN cargo test

CMD cargo run
