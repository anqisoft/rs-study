if not exist example_147 (cargo new example_147)
cd example_147 && cargo add tokio futures_util
copy /y ..\147.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause