if not exist example_135 (cargo new example_135)
cd example_135 && cargo add futures
if not exist src\main.rs (copy /y ..\135.rs src\main.rs)

cargo run >debug.log && start "" notepad debug.log && pause