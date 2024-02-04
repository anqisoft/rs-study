if not exist example_114 (cargo new example_114)
cd example_114 && cargo add thiserror
if not exist src\main.rs (copy /y ..\114.rs src\main.rs)
cargo run > debug.log & start "" notepad debug.log & pause