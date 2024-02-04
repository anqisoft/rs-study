if not exist example_137 (cargo new example_137)
cd example_137 && cargo add tokio
if not exist src\main.rs (copy /y ..\137.rs src\main.rs)
cargo run > run.log & start "" notepad run.log & pause