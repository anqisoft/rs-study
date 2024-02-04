// // eg15-21
// #[cfg(test)]
// mod tests {
//     // use super::*;
//     use crate::eg20::{LimitTracker, Messenger};

//     struct MockMessenger {
//         sent_messages: Vec<String>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: vec![],
//             }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.push(String::from(message));
//             /*
//                 error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
//                 --> src\eg20\eg21.rs:21:13
//                 |
//                 21 |             self.sent_messages.push(String::from(message));
//                 |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//                 |
//                 help: consider changing this to be a mutable reference
//                 --> src\eg20\mod.rs:3:13
//                 |
//                 3  |     fn send(&mut self, msg: &str);
//             */
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.len(), 1);
//     }
// }
