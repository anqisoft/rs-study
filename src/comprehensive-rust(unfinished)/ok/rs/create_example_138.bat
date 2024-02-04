if not exist example_138 (cargo new example_138)
cd example_138 && cargo add tokio
copy /y ..\138.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause