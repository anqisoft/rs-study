@ECHO OFF
setlocal enabledelayedexpansion
cd..
set pwd=%cd%

for /L %%c in (1, 1, 20) do (
  set /a chapter=%%c
  set ok=false
  if "!chapter!"== "3" (set ok=true)
  if "!chapter!"=="8" (set ok=true)
  if "!chapter!"=="20" (set ok=true)
  rem echo %%c !chapter! !ok!
  
  if "!ok!"=="true" (
    if !chapter! lss 10 (
      set project_name=chapter0!chapter!_exercies
    ) else (
      set project_name=chapter!chapter!_exercies
    )

    REM echo !project_name!
    if not exist !project_name! (cargo new !project_name!)
    cd !project_name!

    set goal=release.bat
    set "result=cls && echo %%time%% >release.txt && cargo build --release >>release.txt" && echo !result!>!goal!
    set "result=echo %%time%% >>release.txt && target\release\!project_name! >>release.txt" && echo !result!>>!goal!
    set "result=echo %%time%% >>release.txt" && echo !result!>>!goal!
    set "result=notepad release.txt" && echo start "" !result!>>!goal!

    set goal=debug.bat
    set "result=cls && echo %%time%% >debug.txt && cargo build >>debug.txt" && echo !result!>!goal!
    set "result=echo %%time%% >>debug.txt && target\debug\!project_name! >>debug.txt" && echo !result!>>!goal!
    set "result=echo %%time%% >>debug.txt" && echo !result!>>!goal!
    set "result=notepad debug.txt" && echo start "" !result!>>!goal!

    set goal=run.bat
    set "result=(cls && cargo run) || pause" && echo !result!>!goal!

    set goal=release_and_debug.bat
    set "result=cls && echo %%time%% >release.txt && cargo build --release >>release.txt" && echo !result!>!goal!
    set "result=echo %%time%% >>release.txt && target\release\!project_name! >>release.txt" && echo !result!>>!goal!
    set "result=echo %%time%% >>release.txt" && echo !result!>>!goal!
    set "result=notepad release.txt" && echo start "" !result!>>!goal!
    echo.>>!goal!
    set "result=cls && echo %%time%% >debug.txt && cargo build >>debug.txt" && echo !result!>>!goal!
    set "result=echo %%time%% >>debug.txt && target\debug\!project_name! >>debug.txt" && echo !result!>>!goal!
    set "result=echo %%time%% >>debug.txt" && echo !result!>>!goal!
    set "result=notepad debug.txt" && echo  start "" !result!>>!goal!

    cd !pwd!
  )
)

pause