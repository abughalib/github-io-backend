# Github IO Messaging Receiver API

## Setup
* Configure .env and define DATABASE_URL accordingly or use Postgresql DB.

## Test
Test if everything is configured properly.
```bash
cargo test --verbose
```

## Deploy on Heroku.
Use emk's Rust.
```bash
git clone https://github.com/abughalib/github-io-backend
heroku create --buildpack emk/rust
heroku buildbpacks:set emk/rust

git add .
git commit -m "init"
git push heroku master
```
