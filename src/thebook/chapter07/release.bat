cls && echo %time% >release.txt && cargo build --release >>release.txt
echo %time% >>release.txt && target\release\chapter07 >>release.txt
echo %time% >>release.txt
start "" notepad release.txt
