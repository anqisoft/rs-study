if not exist example_118 (cargo new example_118)
cd example_118
if not exist src\main.rs (copy /y ..\118.rs src\main.rs)

cargo test >test.log && start "" notepad test.log && pause