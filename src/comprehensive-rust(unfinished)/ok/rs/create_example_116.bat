if not exist example_116 (cargo new example_116)
cd example_116
if not exist src\main.rs (copy /y ..\116.rs src\main.rs)
cargo test >test.log && start "" notepad test.log && pause