if not exist example_145 (cargo new example_145)
cd example_145 && cargo add tokio
copy /y ..\145.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause