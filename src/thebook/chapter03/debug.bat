cls && echo %time% >debug.txt && cargo build >>debug.txt
echo %time% >>debug.txt && target\debug\chapter03 >>debug.txt
echo %time% >>debug.txt
start "" notepad debug.txt
