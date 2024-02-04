if not exist example_050 (cargo new example_050)
cd example_050 && cargo add regex
if not exist src\main.rs (copy /y ..\050.rs src\main.rs)
cargo run > debug.log & start "" notepad debug.log & pause