FROM rust:latest
RUN apt install gcc curl libc6-dev libsqlite3-dev
WORKDIR /app
COPY . .
RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel migration run
CMD ["./target/release/abughalib-github"]
