cargo run --features json-file-storage -- \
--title coding \
--status pending
cargo run --features json-file-storage -- \
--title washing \
--status done

(result in task.json)
===
cargo run -p to-do-actix-server