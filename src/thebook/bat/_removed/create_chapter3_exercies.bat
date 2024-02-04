cargo new chapter3_exercies && cd chapter3_exercies
exit

cls && echo %time% >release.txt && cargo build --release >>release.txt 
echo %time% >>release.txt && target\release\chapter3_exercies >>release.txt
echo %time% >>release.txt

cls && echo %time% >debug.txt && cargo build >>debug.txt
echo %time% >>debug.txt && target\debug\chapter3_exercies >>debug.txt
echo %time% >>debug.txt

cls && cargo run

cls && echo %time% >release.txt && cargo build --release >>release.txt 
echo %time% >>release.txt && target\release\chapter3_exercies >>release.txt
echo %time% >>release.txt
cls && echo %time% >debug.txt && cargo build >>debug.txt
echo %time% >>debug.txt && target\debug\chapter3_exercies >>debug.txt
echo %time% >>debug.txt

cd TCO_failed
cls && rustc failed__fibonacci_use_tail_recursion.rs && failed__fibonacci_use_tail_recursion>failed__fibonacci_use_tail_recursion.txt &&  start "" notepad failed__fibonacci_use_tail_recursion.txt
cls && rustc failed__fibonacci_use_tail_recursion2.rs && failed__fibonacci_use_tail_recursion2>failed__fibonacci_use_tail_recursion2.txt && start "" notepad failed__fibonacci_use_tail_recursion2.txt
