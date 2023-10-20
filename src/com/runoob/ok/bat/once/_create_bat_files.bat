@ECHO OFF
CD..

setlocal enabledelayedexpansion

REM sample: (cd ..\debug && rustc ..\001.rs && 001) & pause
FOR /L %%A IN (1, 1, 100) DO (
  set name=00%%A
  set name=!name:~-3!
  REM set "content=(cd ..\debug && rustc ..\!name!.rs && !name!>..\debug\!name!.txt && start "" notepad ..\debug\!name!.txt) || pause"
  set "content=(cd ..\debug && rustc ..\!name!.rs && !name!>..\debug\!name!.txt && start "" notepad ..\debug\!name!.txt) & pause"
  echo !content! >!name!.bat
)

::PAUSE