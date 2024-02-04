@ECHO OFF
if "%%1"=="" (exit)

set project_name=%%1
if not exist %project_name% (cargo new %project_name%)
cd %project_name%

setlocal enabledelayedexpansion

set goal=release.bat
set "result=cls && echo %%time%% >release.txt && cargo build --release >>release.txt" && echo !result!>!goal!
set "result=echo %%time%% >>release.txt && target\release\%project_name% >>release.txt" && echo !result!>>!goal!
set "result=echo %%time%% >>release.txt" && echo !result!>>!goal!
set "result=notepad release.txt" && echo start "" !result!>>!goal!

set goal=debug.bat
set "result=cls && echo %%time%% >debug.txt && cargo build >>debug.txt" && echo !result!>!goal!
set "result=echo %%time%% >>debug.txt && target\debug\%project_name% >>debug.txt" && echo !result!>>!goal!
set "result=echo %%time%% >>debug.txt" && echo !result!>>!goal!
set "result=notepad debug.txt" && echo start "" !result!>>!goal!

set goal=run.bat
set "result=cls && cargo run" && echo !result!>!goal!

set goal=release_and_debug.bat
set "result=cls && echo %%time%% >release.txt && cargo build --release >>release.txt" && echo !result!>!goal!
set "result=echo %%time%% >>release.txt && target\release\%project_name% >>release.txt" && echo !result!>>!goal!
set "result=echo %%time%% >>release.txt" && echo !result!>>!goal!
set "result=notepad release.txt" && echo start "" !result!>>!goal!
echo.>>!goal!
set "result=cls && echo %%time%% >debug.txt && cargo build >>debug.txt" && echo !result!>>!goal!
set "result=echo %%time%% >>debug.txt && target\debug\%project_name% >>debug.txt" && echo !result!>>!goal!
set "result=echo %%time%% >>debug.txt" && echo !result!>>!goal!
set "result=notepad debug.txt" && echo  start "" !result!>>!goal!
