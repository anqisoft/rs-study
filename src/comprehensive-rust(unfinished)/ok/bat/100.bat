(cd ..\debug && rustc --edition 2021 ..\rs\100.rs && 100>..\debug\100.txt && start "" notepad ..\debug\100.txt) & pause 
