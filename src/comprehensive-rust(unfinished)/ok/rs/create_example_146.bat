if not exist example_146 (cargo new example_146)
cd example_146 && cargo add tokio
copy /y ..\146.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause