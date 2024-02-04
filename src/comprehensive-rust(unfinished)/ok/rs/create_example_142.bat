if not exist example_142 (cargo new example_142)
cd example_142 && cargo add tokio futures
copy /y ..\142.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause