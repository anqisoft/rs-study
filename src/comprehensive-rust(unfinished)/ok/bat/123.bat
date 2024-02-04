(cd ..\debug && rustc --edition 2021 ..\rs\123.rs && 123>..\debug\123.txt && start "" notepad ..\debug\123.txt) & pause 
