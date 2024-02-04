@ECHO OFF
CD..

setlocal enabledelayedexpansion

REM sample: (cd ..\debug && rustc ..\rs\001.rs && 001) & pause
FOR /L %%A IN (1, 1, 150) DO (
  set name=00%%A
  set name=!name:~-3!
  if not exist !filename! (
    REM set "content=(cd ..\debug && rustc ..\rs\!name!.rs && !name!>..\debug\!name!.txt && start "" notepad ..\debug\!name!.txt) || pause"
    set "content=(cd ..\debug && rustc --edition 2021 ..\rs\!name!.rs && !name!>..\debug\!name!.txt && start "" notepad ..\debug\!name!.txt) & pause"
    echo !content! >!name!.bat
  )
)

::PAUSE