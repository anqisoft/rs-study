cls && echo %time% >release.txt && cargo build --release >>release.txt
echo %time% >>release.txt && target\release\chapter17 >>release.txt
echo %time% >>release.txt
start "" notepad release.txt

cls && echo %time% >debug.txt && cargo build >>debug.txt
echo %time% >>debug.txt && target\debug\chapter17 >>debug.txt
echo %time% >>debug.txt
 start "" notepad debug.txt
