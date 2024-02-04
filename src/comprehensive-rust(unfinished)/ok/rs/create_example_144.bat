if not exist example_144 (cargo new example_144)
cd example_144 && cargo add tokio async_trait
copy /y ..\144.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause