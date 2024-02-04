if not exist example_140 (cargo new example_140)
cd example_140 && cargo add anyhow futures  reqwest tokio
copy /y ..\140.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause