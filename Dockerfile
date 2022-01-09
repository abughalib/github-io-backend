FROM rust:1.57-buster
RUN apt install gcc curl libc6-dev libsqlite3-dev
WORKDIR /app
COPY . .
RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel migration run
RUN rm -rf /var/cache/apk/*
CMD ["./target/release/abughalib-github"]
