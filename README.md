# Rust API
> Boilerplate Rust API 🦀


## Build Setup

``` bash
# install dependencies
cargo build

# docker container
docker compose up -d

# migrations
sqlx migrate run

# run aplication
cargo watch -q -c -w src/ -x run
```
