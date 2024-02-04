// eg12-13
use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

// pub fn run_imcompleted(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
//     println!("With text:\n{contents}");

//     Ok(())
// }

// #[cfg(test)]
// mod tests_eg12_15 {
//     use super::*;

//     // eg12-15
//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
//   Rust:
//   safe, fast, productive.
//   Pick three.";
//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
// }
/*
error[E0425]: cannot find function `search` in this scope
  --> src\lib.rs:39:53
   |
39 |         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
   |                                                     ^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
*/

// #[cfg(test)]
// mod tests_eg12_16 {
//     #[allow(unused_imports)]
//     use super::*;

//     // eg12-16
//     #[allow(unused_variables)]
//     pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//         vec![]
//     }

//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";
//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
// }
/*
running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----
thread 'tests::one_result' panicked at src\lib.rs:71:9:
assertion `left == right` failed
  left: ["safe, fast, productive."]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/

// eg12-17, eg12-18, eg12-19
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
/* cargo test one_result
running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\debug\deps\minigrep-4a1d63580b704c6d.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/
/* cargo test case
running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\debug\deps\minigrep-4a1d63580b704c6d.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");

    // println!("{}", search(config.query, contents));
    // for result in search(config.query.as_str(), contents.as_str()) {
    for result in search(&config.query, &contents) {
        println!("{result}");
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // temporary value dropped while borrowed
    // let query = query.to_lowercase().as_str();

    let query = query.to_lowercase();

    let mut results: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct ConfigNew {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

use std::env;
impl ConfigNew {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(ConfigNew {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run_new(config: ConfigNew) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}
