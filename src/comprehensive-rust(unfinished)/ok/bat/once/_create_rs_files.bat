@ECHO OFF
CD ..\..\rs\

setlocal enabledelayedexpansion

FOR /L %%A IN (1, 1, 150) DO (
  set name=00%%A
  set name=!name:~-3!
  set filename=!name!.rs
  rem echo !filename!

  REM Please use the original string.
  set "content_line1=fn main() {"

  REM Please use ^!, and can use ^( ^"
  REM The next three statements have same effects.
  REM set "content_line2_seg1=    println^!^(^""
  REM set "content_line2_seg1=    println^!(^""
  REM set "content_line2_seg1=    println^!(""
  REM Use simplest one:
  set "content_line2_seg1=    println^!(""

  REM Please use ^!
  set "content_line2_seg2=Hello, Rustaceans^! Say by "

  REM It's ok: set content_line2_seg3=.");
  REM It's not work:  set "content_line2_seg3=.")^;"
  REM It's ok: set "content_line2_seg3=."^);"
  REM Please use: ^), not use ^"
  set "content_line2_seg3=."^);"

  REM if 1==1 (
  if not exist !filename! (
    rem // 002.rs
    rem // 
    rem
    echo // !filename!>!filename!
    echo // >>!filename!
    echo.>>!filename!

    rem fn main() {
    rem   println!("Hello, Rustaceans! Say by 002.rs.");
    rem }

    echo !content_line1!>>!filename!
    echo !content_line2_seg1!!content_line2_seg2!!filename!!content_line2_seg3!>>!filename!
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