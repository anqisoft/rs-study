if not exist example_034 (cargo new example_034)
cd example_034 && cargo add rand
if not exist src\main.rs (copy /y ..\034.rs src\main.rs)
cargo run > debug.log & start "" notepad debug.log & pause