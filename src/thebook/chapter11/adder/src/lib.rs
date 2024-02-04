pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
 * <en>https://doc.rust-lang.org/book/ch11-01-writing-tests.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch11-01-writing-tests.html</tw>
 */
#[cfg(test)]
mod chapter11_1 {
    use super::*;

    // eg11-1
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    /* eg11-2
        running 1 test
        test tests::it_works ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        Doc-tests adder

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    */

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    /* test tests::exploration ... ok */

    // // eg11-3
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    /* eg11-4
        test tests::another ... FAILED

        failures:

        ---- tests::another stdout ----
        thread 'tests::another' panicked at src\lib.rs:43:9:
        Make this test fail
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    // eg11-6
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));

        // next test
        assert!(!smaller.can_hold(&larger));
    }

    // p247b
    impl Rectangle {
        #[allow(dead_code)]
        fn can_hold_with_bug(&self, other: &Rectangle) -> bool {
            // right
            // self.width > other.width && self.height > other.height
            // to wrong
            self.width < other.width && self.height > other.height
        }
    }
    // #[test]
    // fn larger_can_hold_smaller_with_bug() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };
    //     assert!(larger.can_hold_with_bug(&smaller));
    //     assert!(!smaller.can_hold_with_bug(&larger));
    // }
    /*
        test tests::larger_can_hold_smaller_with_bug ... FAILED

        failures:

        ---- tests::larger_can_hold_smaller_with_bug stdout ----
        thread 'tests::larger_can_hold_smaller_with_bug' panicked at src\lib.rs:93:9:
        assertion failed: larger.can_hold_with_bug(&smaller)
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    // eg11-7
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    /* test tests::it_adds_two ... ok */

    // next
    #[allow(dead_code)]
    pub fn add_two_with_bug(a: i32) -> i32 {
        a + 3
    }
    // #[test]
    // fn it_adds_two_with_bug() {
    //     assert_eq!(4, add_two_with_bug(2));
    // }
    /*
       test tests::it_adds_two_with_bug ... FAILED
       failures:
       ---- tests::it_adds_two_with_bug stdout ----
       thread 'tests::it_adds_two_with_bug' panicked at src\lib.rs:107:9:
       assertion `left == right` failed
       left: 4
       right: 5
    */

    // 11.1.4
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[allow(unused_variables)]
    #[allow(dead_code)]
    pub fn greeting_with_bug(name: &str) -> String {
        String::from("Hello!")
    }
    // #[test]
    // fn greeting_contains_name_with_bug() {
    //     let result = greeting_with_bug("Carol");
    //     assert!(result.contains("Carol"));
    // }
    /*
        test tests::greeting_contains_name_with_bug ... FAILED

        failures:

        ---- tests::greeting_contains_name_with_bug stdout ----
        thread 'tests::greeting_contains_name_with_bug' panicked at src\lib.rs:154:9:
        assertion failed: result.contains(\"Carol\")
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
    // #[test]
    // fn greeting_contains_name_more_info_with_bug() {
    //     let result = greeting_with_bug("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{}`",
    //         result
    //     );
    // }
    /*
        test tests::greeting_contains_name_more_info_with_bug ... FAILED

        failures:

        ---- tests::greeting_contains_name_more_info_with_bug stdout ----
        thread 'tests::greeting_contains_name_more_info_with_bug' panicked at src\lib.rs:169:9:
        Greeting did not contain name, value was `Hello!`
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    // 11.1.5 eg11-8
    #[allow(dead_code)]
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
    }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    /* test tests::greater_than_100 - should panic ... ok */

    // next
    impl Guess {
        #[allow(dead_code)]
        pub fn new_with_bug(value: i32) -> Guess {
            if value < 1 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
    }
    // #[test]
    // #[should_panic]
    // fn greater_than_100_with_bug() {
    //     Guess::new_with_bug(200);
    // }
    /*
        test tests::greater_than_100_with_bug - should panic ... FAILED

        failures:

        ---- tests::greater_than_100_with_bug stdout ----
        note: test did not panic as expected
    */

    // eg11-9
    impl Guess {
        pub fn new_more_accurate(value: i32) -> Guess {
            if value < 1 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
            Guess { value }
        }
    }
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_more_accurate() {
        Guess::new_more_accurate(200);
    }
    /* test tests::greater_than_100_more_accurate - should panic ... ok */
    // // anqi, with bug:
    // #[test]
    // #[should_panic(expected = "great than or equal to 1")]
    // fn less_than_1_more_accurate_panic_message_not_contains_expected_string() {
    //     Guess::new_more_accurate(0);
    // }
    /*
        test tests::less_than_1_more_accurate_panic_message_not_contains_expected_string - should panic ... FAILED
        failures:

        ---- tests::less_than_1_more_accurate_panic_message_not_contains_expected_string stdout ----
        thread 'tests::less_than_1_more_accurate_panic_message_not_contains_expected_string' panicked at src\lib.rs:234:17:
        Guess value must be between 1 and 100, got 0.
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        note: panic did not contain expected string
            panic message: `"Guess value must be between 1 and 100, got 0."`,
        expected substring: `"great than or equal to 1"`
    */
    // anqi, ok:
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn less_than_1_more_accurate() {
        Guess::new_more_accurate(0);
    }
    /* test tests::less_than_1_more_accurate - should panic ... ok */

    // next
    impl Guess {
        #[allow(dead_code)]
        pub fn new_more_accurate_with_bug_because_swapped_order(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            }
            Guess { value }
        }
    }
    // #[test]
    // #[should_panic(expected = "less than or equal to 100")]
    // fn greater_than_100_more_accurate_with_bug_because_swapped_order() {
    //     Guess::new_more_accurate_with_bug_because_swapped_order(200);
    // }
    /*
        test tests::greater_than_100_more_accurate_with_bug_because_swapped_order - should panic ... FAILED

        failures:

        ---- tests::greater_than_100_more_accurate_with_bug_because_swapped_order stdout ----
        thread 'tests::greater_than_100_more_accurate_with_bug_because_swapped_order' panicked at src\lib.rs:286:17:
        Guess value must be greater than or equal to 1, got 200.
        note: panic did not contain expected string
            panic message: `"Guess value must be greater than or equal to 1, got 200."`,
        expected substring: `"less than or equal to 100"
    */

    #[test]
    fn it_works_use_error() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    /* test tests::it_works_use_error ... ok */

    // anqi
    fn odd_ok_and_even_error(value: i32) -> Result<(), String> {
        if value % 2 == 1 {
            Ok(())
        } else {
            Err(String::from("An even number produces an error"))
        }
    }
    #[test]
    fn test_odd_ok_and_even_error() {
        for i in 0..6 {
            let value = odd_ok_and_even_error(i);
            if i % 2 == 1 {
                assert!(!value.is_err())
            } else {
                assert!(value.is_err())
            }
        }
    }
    /* test tests::test_odd_ok_and_even_error ... ok */
}

#[cfg(test)]
mod chapter11_2 {
    #[allow(unused_imports)]
    use super::*;

    /*
     * <en>https://doc.rust-lang.org/book/ch11-02-running-tests.html</en>
     * <cn>https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html</cn>
     * <tw>https://rust-lang.tw/book-tw/ch11-02-running-tests.html</tw>
     */

    // eg11-10
    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {}", a);
        10
    }
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    /* test tests::this_test_will_pass ... ok */
    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }
    /* cargo test
        test tests::this_test_will_fail ... FAILED

        failures:

        ---- tests::this_test_will_fail stdout ----
        I got the value 8
        thread 'tests::this_test_will_fail' panicked at src\lib.rs:362:9:
        assertion `left == right` failed
        left: 5
        right: 10
    */
    /* cargo test -- --show-output
        test tests::this_test_will_fail ... FAILED
        test tests::this_test_will_pass ... ok

        ---- tests::this_test_will_pass stdout ----
        I got the value 4

        successes:
            tests::this_test_will_pass

        failures:

        ---- tests::this_test_will_fail stdout ----
        I got the value 8
        thread 'tests::this_test_will_fail' panicked at src\lib.rs:363:9:
        assertion `left == right` failed
        left: 5
        right: 10


        failures:
            tests::this_test_will_fail
    */

    // eg11-11
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    /* cargo test
        running 3 tests
        test add_three_and_two ... ok
        test add_two_and_two ... ok
        test one_hundred ... ok

        test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

        Doc-tests adder

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
    */
    /* cargo test add_three_and_two add_two_and_two one_hundred
        error: unexpected argument 'add_two_and_two' found

        Usage: cargo.exe test [OPTIONS] [TESTNAME] [-- [args]...]

        For more information, try '--help'.
    */
    /* cargo test one_hundred
        running 1 test
        test one_hundred ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s
    */
    /* cargo test add
        running 3 tests
        test chapter11_1::it_adds_two ... ok
        test chapter11_2::add_three_and_two ... ok
        test chapter11_2::add_two_and_two ... ok

        test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s
    */

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
    /* cargo test
        test chapter11_2::expensive_test ... ignored
    */
    /* cargo test −− −−ignored
        running 1 test
        test chapter11_2::expensive_test ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

        Doc-tests adder

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    */
}

/*
 * <en>https://doc.rust-lang.org/book/ch11-03-test-organization.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch11-03-test-organization.html</tw>
 */
mod chapter11_3 {
    #[allow(unused_imports)]
    use super::*;

    // eg11-12
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
    /* test chapter11_3::internal ... ok */

    // eg11-13: ../tests/integration_test.rs
}

/*
* <en>https://doc.rust-lang.org/book/ch11-01-writing-tests.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch11-01-writing-tests.html</tw>
*/

// eg11-5
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
 * <en>https://doc.rust-lang.org/book/ch11-02-running-tests.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch11-02-running-tests.html</tw>
 */

/*
 * <en>https://doc.rust-lang.org/book/ch11-03-test-organization.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch11-03-test-organization.html</tw>
 */
// eg11-12
#[allow(dead_code)]
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// eg11-13
pub fn add_two(a: i32) -> i32 {
    a + 2
}
