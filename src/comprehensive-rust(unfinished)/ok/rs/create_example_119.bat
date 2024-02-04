if not exist example_119 (cargo new example_119)
cd example_119
::if not exist src\main.rs (copy /y ..\119.rs src\main.rs)

cargo test >test.log && start "" notepad test.log && pause