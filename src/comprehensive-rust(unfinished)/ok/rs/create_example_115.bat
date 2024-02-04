if not exist example_115 (cargo new example_115)
cd example_115 && cargo add anyhow
if not exist src\main.rs (copy /y ..\115.rs src\main.rs)
cargo run > debug.log & start "" notepad debug.log & pause