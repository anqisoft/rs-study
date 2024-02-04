if not exist example_117 (cargo new example_117)
cd example_117
if not exist src\main.rs (copy /y ..\117.rs src\main.rs)

cargo test >test.log && start "" notepad test.log
cargo run >debug.log && start "" notepad debug.log
pause