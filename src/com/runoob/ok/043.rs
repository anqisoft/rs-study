// 043.rs
// https://www.runoob.com/rust/rust-error-handle.html

fn main() {
    panic!("error occured");
    println!("Hello, Rust");
}

/* result:
thread 'main' panicked at ..\043.rs:5:5:
error occured
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

/* warnings:
warning: unreachable statement
 --> ..\043.rs:6:5
  |
5 |     panic!("error occured");
  |     ----------------------- any code following this expression is unreachable
6 |     println!("Hello, Rust");
  |     ^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 1 warning emitted
*/

/* RUST_BACKTRACE=1
warning: unreachable statement
 --> ..\043.rs:6:5
  |
5 |     panic!("error occured");
  |     ----------------------- any code following this expression is unreachable
6 |     println!("Hello, Rust");
  |     ^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 1 warning emitted

thread 'main' panicked at ..\043.rs:5:5:
error occured
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/

/*
rustc : warning: unreachable statement
At line:1 char:1
+ rustc ..\043.rs
+ ~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: unreachable statement:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError

 --> ..\043.rs:6:5
  |
5 |     panic!("error occured");
  |     ----------------------- any code following this expression is unreachable
6 |     println!("Hello, Rust");
  |     ^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 1 warning emitted
Start-Process : Cannot validate argument on parameter 'FilePath'. The argument is null or empty. Provide an argument that is not
null or empty, and then try the command again.
At line:3 char:7
+ start "" notepad ..\debug\043.txt
+       ~~
    + CategoryInfo          : InvalidData: (:) [Start-Process], ParameterBindingValidationException
    + FullyQualifiedErrorId : ParameterArgumentValidationError,Microsoft.PowerShell.Commands.StartProcessCommand
 */

/* Powershell $env:RUST_BACKTRACE=1;
rustc : warning: unreachable statement
At line:1 char:1
+ rustc ..\043.rs
+ ~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: unreachable statement:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError

 --> ..\043.rs:6:5
  |
5 |     panic!("error occured");
  |     ----------------------- any code following this expression is unreachable
6 |     println!("Hello, Rust");
  |     ^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 1 warning emitted
Start-Process : Cannot validate argument on parameter 'FilePath'. The argument is null or empty. Provide an argument that is not
null or empty, and then try the command again.
At line:3 char:7
+ start "" notepad ..\debug\043.txt
+       ~~
    + CategoryInfo          : InvalidData: (:) [Start-Process], ParameterBindingValidationException
    + FullyQualifiedErrorId : ParameterArgumentValidationError,Microsoft.PowerShell.Commands.StartProcessCommand
*/

/* Powershell $env:RUST_BACKTRACE=full;
full : The term 'full' is not recognized as the name of a cmdlet, function, script file, or operable program. Check the spelling of
the name, or if a path was included, verify that the path is correct and try again.
At line:1 char:21
+ $env:RUST_BACKTRACE=full;
+                     ~~~~
    + CategoryInfo          : ObjectNotFound: (full:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
*/