(cd ..\debug && rustc -L ..\example_140\target\debug\build\ --edition 2021 ..\rs\142.rs && 142>..\debug\142.txt && start "" notepad ..\debug\142.txt) & pause 
