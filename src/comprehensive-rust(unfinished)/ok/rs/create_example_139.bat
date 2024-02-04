if not exist example_139 (cargo new example_139)
cd example_139 && cargo add tokio
copy /y ..\139.rs src\main.rs
cargo run > run.log & start "" notepad run.log & pause