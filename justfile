run DAY:
    cargo run --bin {{DAY}}

run-all:
    cargo run --release --bin runner

test:
    cargo test --lib

watch:
    bacon test

bench-all:
    cargo bench --bench all-days

bench DAY:
    cargo bench --bench all-days -- {{DAY}}
