@ECHO OFF
set filename=show_used_time

cls 
if not exist ..\target\debug\ (md ..\target\debug\)
if not exist ..\test_log\ (md ..\test_log\)
(cd ..\target\debug\ && rustc ..\..\src\%filename%.rs && %filename% >..\..\test_log\%filename%.txt && start "" ..\..\test_log\%filename%.txt) || pause