@ECHO OFF
:: <en>The content will be confusing. For example, 
:: the first line of release.bat is echo %time% >>debug.txt, 
:: while the first line of debug.bat is echo %time% >>release.txt, 
:: and the first line of run.bat is echo %time% >> debug.txt, 
:: the first line of release_and_debug.bat is cls && cargo run.
:: </en>
:: <cn>���ݻ���ң�����
:: release.bat��һ����echo %time% >>debug.txt��
:: ��debug.bat��һ����echo %time% >>release.txt��
:: run.batΪecho %time% >>debug.txt��
:: release_and_debug.bat��һ����cls && cargo run��</cn>
:: <tw>���ݕ��e�y������
:: release.bat��һ����echo %time% >>debug.txt��
:: ��debug.bat��һ����echo %time% >>release.txt��
:: run.bat��echo %time% >> debug.txt��
:: release_and_debug.bat��һ����cls && cargo run��
:: </tw>
exit

set project_name=chapter4
if not exist %project_name% (cargo new %project_name%)
cd %project_name%

set goal=release.bat
set "result=cls ^&^& echo %%time%% ^>release.txt ^&^& cargo build --release ^>^>release.txt" && echo %result%>%goal%
set "result=echo %%time%% ^>^>release.txt ^&^& target\release\%project_name% ^>^>release.txt" && echo %result%>>%goal%
set "result=echo %%time%% ^>^>release.txt" && echo %result%>>%goal%

set goal=debug.bat
set "result=cls ^&^& echo %%time%% ^>debug.txt ^&^& cargo build ^>^>debug.txt" && echo %result%>%goal%
set "result=echo %%time%% ^>^>debug.txt ^&^& target\debug\%project_name% ^>^>debug.txt" && echo %result%>>%goal%
set "result=echo %%time%% ^>^>debug.txt" && echo %result%>>%goal%

set goal=run.bat
set "result=cls ^&^& cargo run" && echo %result%>%goal%

set goal=release_and_debug.bat
set "result=cls ^&^& echo %%time%% ^>release.txt ^&^& cargo build --release ^>^>release.txt" && echo %result%>%goal%
set "result=echo %%time%% ^>^>release.txt ^&^& target\release\%project_name% ^>^>release.txt" && echo %result%>>%goal%
set "result=echo %%time%% ^>^>release.txt" && echo %result%>>%goal%
set "result=cls ^&^& echo %%time%% ^>debug.txt ^&^& cargo build ^>^>debug.txt" && echo %result%>>%goal%
set "result=echo %%time%% ^>^>debug.txt ^&^& target\debug\%project_name% ^>^>debug.txt" && echo %result%>>%goal%
set "result=echo %%time%% ^>^>debug.txt" && echo %result%>>%goal%
