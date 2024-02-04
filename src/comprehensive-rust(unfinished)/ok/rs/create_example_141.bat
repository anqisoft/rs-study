if not exist example_141 (cargo new example_141)
cd example_141 && cargo add tokio
copy /y ..\141.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause