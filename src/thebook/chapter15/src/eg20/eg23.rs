// // eg15-23
// #[cfg(test)]
// mod tests_with_error_2 {
//     // use super::*;
//     use crate::eg20::Messenger;
//     use std::cell::RefCell;

//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.borrow_mut().push(String::from(message));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mut one_borrow = self.sent_messages.borrow_mut();
//         let mut two_borrow = self.sent_messages.borrow_mut();

//         one_borrow.push(String::from(message));
//         two_borrow.push(String::from(message));
//     }
// }
// /*
// error[E0424]: expected value, found module `self`
//   --> src\eg20\eg23.rs:28:30
//    |
// 27 |     fn it_sends_an_over_75_percent_warning_message() {
//    |        ------------------------------------------- this function can't have a `self` parameter
// 28 |         let mut one_borrow = self.sent_messages.borrow_mut();
//    |                              ^^^^ `self` value is a keyword only available in methods with a `self` parameter

// error[E0424]: expected value, found module `self`
//   --> src\eg20\eg23.rs:29:30
//    |
// 27 |     fn it_sends_an_over_75_percent_warning_message() {
//    |        ------------------------------------------- this function can't have a `self` parameter
// 28 |         let mut one_borrow = self.sent_messages.borrow_mut();
// 29 |         let mut two_borrow = self.sent_messages.borrow_mut();
//    |                              ^^^^ `self` value is a keyword only available in methods with a `self` parameter

// error[E0425]: cannot find value `message` in this scope
//   --> src\eg20\eg23.rs:31:38
//    |
// 31 |         one_borrow.push(String::from(message));
//    |                                      ^^^^^^^ not found in this scope

// error[E0425]: cannot find value `message` in this scope
//   --> src\eg20\eg23.rs:32:38
//    |
// 32 |         two_borrow.push(String::from(message));
//    |                                      ^^^^^^^ not found in this scope
// */
