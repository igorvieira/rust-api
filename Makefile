run:
		cargo run
watcher:
		cargo watch -q -c -w src/ -x run
migration_up:
		sqlx migrate run
migration_down:
		sqlx migrate revert
