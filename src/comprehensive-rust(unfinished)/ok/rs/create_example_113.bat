if not exist example_113 (cargo new example_113)
cd example_113 && cargo add thiserror
if not exist src\main.rs (copy /y ..\113.rs src\main.rs)
cargo run > debug.log & start "" notepad debug.log & pause