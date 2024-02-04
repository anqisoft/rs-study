use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
/* cargo test
running 16 tests
test chapter11_1::exploration ... ok
test chapter11_1::greater_than_100 - should panic ... ok
test chapter11_1::greater_than_100_more_accurate - should panic ... ok
test chapter11_1::greeting_contains_name ... ok
test chapter11_1::it_works ... ok
test chapter11_1::larger_can_hold_smaller ... ok
test chapter11_1::it_adds_two ... ok
test chapter11_1::it_works_use_error ... ok
test chapter11_1::less_than_1_more_accurate - should panic ... ok
test chapter11_1::test_odd_ok_and_even_error ... ok
test chapter11_2::add_three_and_two ... ok
test chapter11_2::add_two_and_two ... ok
test chapter11_2::expensive_test ... ignored
test chapter11_2::one_hundred ... ok
test chapter11_2::this_test_will_pass ... ok
test chapter11_3::internal ... ok

test result: ok. 15 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests\integration_test.rs (target\debug\deps\integration_test-17834ff8e2a54e55.exe)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/

/* cargo test it_adds_two
running 1 test
test chapter11_1::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests\integration_test.rs (target\debug\deps\integration_test-17834ff8e2a54e55.exe)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/

/* cargo test --test integration_test
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/

mod common;
#[test]
fn it_adds_two_new() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
/* cargo test --test integration_test
running 2 tests
test it_adds_two ... ok
test it_adds_two_new ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/