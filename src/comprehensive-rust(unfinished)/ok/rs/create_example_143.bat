if not exist example_143 (cargo new example_143)
cd example_143 && cargo add tokio
copy /y ..\143.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause