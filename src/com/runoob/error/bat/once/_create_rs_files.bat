@ECHO OFF
CD ..\..\

setlocal enabledelayedexpansion

FOR /L %%A IN (1, 1, 100) DO (
  set name=00%%A
  set name=!name:~-3!
  set filename=!name!.rs
  rem echo !filename!

  REM Please use the original string.
  set "content_line1=fn main() {"

  if not exist !filename! (
    rem // error/001.rs
    rem // 
    rem
    echo // error/!filename!>!filename!
    echo // >>!filename!
    echo.>>!filename!

    rem fn main() {
    rem   
    rem }

    echo !content_line1!>>!filename!
    echo.>>!filename!
    echo }>>!filename!

    rem
    rem /* result:
    rem
    rem */
    echo.>>!filename!
    echo /* result:>>!filename!
    echo.>>!filename!
    echo */>>!filename!
  )
)

::PAUSE