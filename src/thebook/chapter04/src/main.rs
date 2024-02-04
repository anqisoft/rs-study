fn p86a() {
    println!("\nIn p86a()");

    // {
    //     let s = "hello";
    // }
    // println!("{s}");
}
/*
error[E0425]: cannot find value `s` in this scope
 --> src\main.rs:7:16
  |
7 |     println!("{s}");
  |                ^
  |
help: the binding `s` is available in a different scope in the same function
 --> src\main.rs:5:13
  |
5 |         let s = "hello";
  |             ^

For more information about this error, try `rustc --explain E0425`.
*/

fn p87a() {
    println!("\nIn p87a()");

    let mut s = String::from("hello");
    s.push_str(", world!");

    // or println!("{s}");
    println!("{}", s);
}

fn p88a() {
    println!("\nIn p88a()");

    let s = String::from("hello");
    println!("The type of String::from(\"hello\") is {}.", type_of(&s));

    let s = "hello";
    println!("The type of \"hello\" is {}.", type_of(&s));
}

fn p88b() {
    println!("\nIn p88b()");

    let x = 5;
    let y = x;
    println!("x is {x}, and its type is {}. y is {y}, and its type is {}.", type_of(&x), type_of(&y));
}

fn p88c() {
    println!("\nIn p88c()");

    let s1 = String::from("hello");
    println!("s1 is {s1}, and its type is {}.", type_of(&s1));

    let s2 = s1;
    println!("s2 is {s2}, and its type is {}.", type_of(&s2));

    // error: 
    // println!("s1 is {s1}, and its type is {}.", type_of(&s1));
    /*
        error[E0382]: borrow of moved value: `s1`
        --> src\main.rs:63:57
        |
        56 |     let s1 = String::from("hello");
        |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        ...
        59 |     let s2 = s1;
        |              -- value moved here
        ...
        63 |     println!("s1 is {s1}, and its type is {}.", type_of(&s1));
        |                                                         ^^ value borrowed here after move
        |
        help: consider cloning the value if the performance cost is acceptable
        |
        59 |     let s2 = s1.clone();
        |                ++++++++

        For more information about this error, try `rustc --explain E0382`.
    */
}

#[allow(unused_variables)]
fn p92a() {
    println!("\nIn p92a()");

    let s1 = String::from("hello");
    let s2 = s1;
    
    // println!("{}, world!", s1);
    /*
        error[E0382]: borrow of moved value: `s1`
        --> src\main.rs:93:28
        |
        90 |     let s1 = String::from("hello");
        |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        91 |     let s2 = s1;
        |              -- value moved here
        92 |
        93 |     println!("{}, world!", s1);
        |                            ^^ value borrowed here after move
        |
        = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
        |
        91 |     let s2 = s1.clone();
        |                ++++++++

        For more information about this error, try `rustc --explain E0382`.
    */
}

fn p94a() {
    println!("\nIn p94a()");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn p94b() {
    println!("\nIn p94b()");

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // println!("x is {x}, and its type is {}. y is {y}, and its type is {}.", type_of(x), type_of(y));
}

fn p95a() {
    println!("\nIn p95a()");

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
}

fn p95b() {
    println!("\nIn p95b()");

    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn p96a() {
    println!("\nIn p96a()");
    
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_p96(s1);
    println!("The length of '{}' is {}.", s2, len);

    // println!("s1 is {s1}");
    /*
        error[E0382]: borrow of moved value: `s1`
        --> src\main.rs:172:21
            |
        168 |     let s1 = String::from("hello");
            |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        169 |     let (s2, len) = calculate_length_p96(s1);
            |                                      -- value moved here
        ...
        172 |     println!("s1 is {s1}");
            |                     ^^^^ value borrowed here after move
            |
        note: consider changing this parameter type in function `calculate_length_p96` to borrow instead if owning the value isn't necessary
        --> src\main.rs:174:24
            |
        201 | fn calculate_length_p96(s: String) -> (String, usize) {
            |    ----------------    ^^^^^^ this parameter takes ownership of the value
            |    |
            |    in this function
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
            |
        169 |     let (s2, len) = calculate_length_p96(s1.clone());
            |                                        ++++++++

        For more information about this error, try `rustc --explain E0382`.
    */
}
fn calculate_length_p96(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn p97a() {
    println!("\nIn p97a()");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn p98a() {
    println!("\nIn p98a()");

    let s1 = String::from("hello");
    // change_p98a(&s1);

    println!("s1 is {s1}");
}
// #[allow(dead_code)]
// fn change_p98a(some_string: &String) {
//     some_string.push_str(", world");
// }
/*
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
   --> src\main.rs:227:5
    |
227 |     some_string.push_str(", world");
    |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    |
help: consider changing this to be a mutable reference
    |
226 | fn change_p98a(some_string: &mut String) {
    |                              +++

For more information about this error, try `rustc --explain E0596`.
*/

fn p99a() {
    println!("\nIn p99a()");

    let mut s = String::from("hello");
    change(&mut s);

    println!("s is {s}");
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn p99b() {
    println!("\nIn p99b()");

    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    /*
        error[E0499]: cannot borrow `s` as mutable more than once at a time
        --> src\main.rs:262:14
            |
        261 |     let r1 = &mut s;
            |              ------ first mutable borrow occurs here
        262 |     let r2 = &mut s;
            |              ^^^^^^ second mutable borrow occurs here
        ...
        264 |     println!("{}, {}", r1, r2);
            |                        -- first borrow later used here

        For more information about this error, try `rustc --explain E0499`.
    */

    s.push_str(", world!");
    println!("s is '{s}'.");
}

fn p100a() {
    println!("\nIn p100a()");

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 is {r1}.");
    }

    let r2 = &mut s;
    println!("r2 is {r2}.");
}

fn p100b() {
    println!("\nIn p100b()");

    let mut s = String::from("hello");

    {
        let r1 = &s; // ok
        let r2 = &s; // ok
        println!("r1 is {r1}, r2 is {r2}");
    }

    {
        let _r1 = &s; // ok
        let _r2 = &s; // ok
    
        let _r3 = &mut s;
    }

    {
        let _r1 = &s;
        let r3 = &mut s;
        println!("r3 is {r3}.");
    }

    // {
    //     let r1 = &s; // ok
    //     let r2 = &s; // ok
    
    //     let r3 = &mut s; // error!!!
    
    //     println!("{r1}, {r2}, and {r3}");
    // }

    // {
    //     let r1 = &s;
    //     let r3 = &mut s;
    //     println!("{r1} and {r3}");
    // }
}
/*
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
   --> src\main.rs:325:18
    |
322 |         let r1 = &s; // ok
    |                  -- immutable borrow occurs here
...
325 |         let r3 = &mut s; // error!!!
    |                  ^^^^^^ mutable borrow occurs here
326 |
327 |         println!("{r1}, {r2}, and {r3}");
    |                   ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`
*/
/*
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
   --> src\main.rs:332:18
    |
331 |         let r1 = &s;
    |                  -- immutable borrow occurs here
332 |         let r3 = &mut s;
    |                  ^^^^^^ mutable borrow occurs here
333 |         println!("{r1} and {r3}");
    |                   ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
*/

fn p101a() {
    println!("\nIn p101a()");

    let mut s = String::from("hello");
    let r1 = &s; 
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s; 
    println!("{}", r3);
}

fn p101b() {
    println!("\nIn p101b()");
    
    // let reference_to_nothing = dangle();
    // println!("reference_to_nothing is {reference_to_nothing}.");
}
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
/*
error[E0106]: missing lifetime specifier
   --> src\main.rs:383:16
    |
383 | fn dangle() -> &String {
    |                ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
    |
383 | fn dangle() -> &'static String {
    |                 +++++++

For more information about this error, try `rustc --explain E0106`.
*/
// fn dangle() -> &'static String {
//     let s = String::from("hello");
//     &s
// }
/*
error[E0515]: cannot return reference to local variable `s`
   --> src\main.rs:404:5
    |
404 |     &s
    |     ^^ returns a reference to data owned by the current function

For more information about this error, try `rustc --explain E0515`.
*/
// fn dangle() -> &'static String {
//     static s: String = String::from("hello");
//     &s
// }
/*
error[E0015]: cannot call non-const fn `<String as From<&str>>::from` in statics
   --> src\main.rs:416:24
    |
416 |     static s: String = String::from("hello");
    |                        ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
    = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell

For more information about this error, try `rustc --explain E0015`.
*/
// fn dangle() -> &'static String {
//     static s: String = "hello";
//     &s
// }
/*
error[E0308]: mismatched types
   --> src\main.rs:432:24
    |
432 |     static s: String = "hello";
    |                        ^^^^^^^- help: try using a conversion method: `.to_string()`
    |                        |
    |                        expected `String`, found `&str`

For more information about this error, try `rustc --explain E0308`.
*/
// fn dangle() -> &'static String {
//     static s: String = "hello".to_string();
//     &s
// }
/*
error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in statics
   --> src\main.rs:447:32
    |
447 |     static s: String = "hello".to_string();
    |                                ^^^^^^^^^^^
    |
    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
    = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell

For more information about this error, try `rustc --explain E0015`.
*/
// fn dangle() -> &'static String {
//     static s: String = Lazy::new(|| "hello".to_string());
//     &s
// }
/*
error[E0433]: failed to resolve: use of undeclared type `Lazy`
   --> src\main.rs:463:24
    |
463 |     static s: String = Lazy::new(|| "hello".to_string());
    |                        ^^^^ use of undeclared type `Lazy`

For more information about this error, try `rustc --explain E0433`.
*/

fn p102a() {
    println!("\nIn p102a()");

    let string = no_dangle();
    println!("string is {string}.");

    let s2 = no_dangle2();
    println!("s2 is {s2}.");
}
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
fn no_dangle2() -> String {
    String::from("hello")
}

fn p104a() {
    // TODO(@anqisoft) Don't use this.
    println!("\nIn p104a()");

    for s in ["test", "test it", "I test it"] {
        println!("first_char(&'{s}') is '{}'.", first_char(&s.to_string()));
    }
}
fn first_char(s: &String) -> String {
    println!("{:?}", s.split(" "));
    s[0..1].to_string()
}

fn p105a() {
    println!("\nIn p105a()");

    for str in ["test", "test it", "I test it"] {
        let mut s = str.to_string();
        let word = first_word_end_position(&s);
        println!("first_word_end_position(&'{s}') is '{word}'.");

        s.clear();
        println!("After clearing the string, the end position of the first word is {word}.");
    }
}
fn first_word_end_position(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn p106a() {
    println!("\nIn p106a()");

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("s is '{s}', hello is '{hello}', and world is '{world}'.");
}

fn p108a() {
    println!("\nIn p106a()");

    for str in ["test", "test it", "I test it"] {
        let s = str.to_string();
        println!("first_word(&'{s}') is '{}', and the second_word(&'{s}') is '{}'.", first_word(&s), second_word(&s));
    }
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    let mut start_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start_index > 0 {
                return &s[start_index..i];
            } else {
                start_index = i + 1;
            }
        }
    }

    if start_index > 0 { &s[start_index..] } else { "" }
}

fn p109a() {
    println!("\nIn p109a()");

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // println!("the first word is: {}", word);
}
/*
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
   --> src\main.rs:580:5
    |
579 |     let word = first_word(&s);
    |                           -- immutable borrow occurs here
580 |     s.clear();
    |     ^^^^^^^^^ mutable borrow occurs here
581 |     println!("the first word is: {}", word);
    |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
*/

fn p109b(){
    println!("\nIn p109b()");

    let s = String::from("test");

    // ok
    {
        let s1 = &s;
        let mut _s2 = &s;
    
        println!("s1 is '{s1}'.");
    }
    /* 
        s1 is 'test'.
    */

    // ok
    // {
    //     let s1 = &s;
    //     let s2 = &s;
    
    //     println!("s1 is '{s1}, s2 is '{s2}'.");
    // }
    /* 
        s1 is 'test, s2 is 'test'.
    */

    // {
    //     let mut s = String::from("test");
    //     let s1 = &s;
    //     let mut _s2 = &mut s;
    //     println!("s1 is '{s1}'.");
    // }
    /*
        error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        --> src\main.rs:627:23
            |
        626 |         let s1 = &s;
            |                  -- immutable borrow occurs here
        627 |         let mut _s2 = &mut s;
            |                       ^^^^^^ mutable borrow occurs here
        628 |         println!("s1 is '{s1}'.");
            |                          ---- immutable borrow later used here

        For more information about this error, try `rustc --explain E0502`.
    */

    // ok
    {
        let mut s = String::from("test");

        let s1 = &s;
        println!("s1 is '{s1}'.");

        let s2 = &mut s;
        s2.push('!');
        println!("s2 is '{s2}'.");
    }
    /*
        s1 is 'test'.
        s2 is 'test'.
    */

    // {
    //     let mut s = String::from("test");
    //     let s1 = &s;
    //     let s2 = &mut s;
    
    //     println!("s1 is '{s1}, s2 is '{s2}'.");
    // }
    /*
        error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        --> src\main.rs:662:22
            |
        661 |         let s1 = &s;
            |                  -- immutable borrow occurs here
        662 |         let s2 = &mut s;
            |                      ^^^^^^ mutable borrow occurs here
        663 |
        664 |         println!("s1 is '{s1}, s2 is '{s2}'.");
            |                          ---- immutable borrow later used here

        For more information about this error, try `rustc --explain E0502`.
    */
}

fn p110a() {
    println!("\nIn p110a()");

    // let my_string = String::from("hello world");
    // println!("my_string is '{}', and its type is {}.", &my_string, type_of(&my_string));
    // println!("first_word(&my_string[0..6]) is {}", first_word(&my_string[0..6]));
    // println!("first_word(&my_string[..]) is {}", first_word(&my_string[..]));
    // println!("first_word(&my_string) is {}", first_word(&my_string));
      
    // let my_string_literal = "hello world";
    // println!("\nmy_string_literal is '{}', and its type is {}.", &my_string_literal, type_of(&my_string_literal));
    // println!("first_word(&my_string_literal[0..6]) is {}", first_word(&my_string_literal[0..6]));
    // println!("first_word(&my_string_literal[..]) is {}", first_word(&my_string_literal[..]));
    // println!("first_word(my_string_literal) is {}", first_word(my_string_literal));
    
    let my_string = String::from("hello world rust");
    println!("my_string is '{}', and its type is {}.", &my_string, type_of(&my_string));
    println!("first_word_p111(&my_string[0..6]) is {}", first_word_p111(&my_string[0..6]));
    println!("first_word_p111(&my_string[..]) is {}", first_word_p111(&my_string[..]));
    println!("first_word_p111(&my_string) is {}", first_word_p111(&my_string));
    println!("second_word_p111(&my_string[0..6]) is {}", second_word_p111(&my_string[0..6]));
    println!("second_word_p111(&my_string[..]) is {}", second_word_p111(&my_string[..]));
    println!("second_word_p111(&my_string) is {}", second_word_p111(&my_string));
      
    let my_string_literal = "hello world rust";
    println!("\nmy_string_literal is '{}', and its type is {}.", &my_string_literal, type_of(&my_string_literal));
    println!("first_word_p111(&my_string_literal[0..6]) is {}", first_word_p111(&my_string_literal[0..6]));
    println!("first_word_p111(&my_string_literal[..]) is {}", first_word_p111(&my_string_literal[..]));
    println!("first_word_p111(my_string_literal) is {}", first_word_p111(my_string_literal));
    println!("\nmy_string_literal is '{}', and its type is {}.", &my_string_literal, type_of(&my_string_literal));
    println!("second_word_p111(&my_string_literal[0..6]) is {}", second_word_p111(&my_string_literal[0..6]));
    println!("second_word_p111(&my_string_literal[..]) is {}", second_word_p111(&my_string_literal[..]));
    println!("second_word_p111(my_string_literal) is {}", second_word_p111(my_string_literal));
}
fn first_word_p111(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn second_word_p111(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut start_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start_index > 0 {
                return &s[start_index..i];
            } else {
                start_index = i + 1;
            }
        }
    }

    if start_index > 0 { &s[start_index..] } else { "" }
}

fn p111a() {
    println!("\nIn p111a()");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter04_main";
    let start_chapter_line = "Chapter 04";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");
        
            p86a();
    
            p87a();
    
            p88a();
            p88b();
            p88c();
    
            p92a();
    
            p94a();
            p94b();
    
            p95a();
            p95b();
    
            p96a();
    
            p97a();
    
            p98a();
    
            p99a();
            p99b();
    
            p100a();
            p100b();
    
            p101a();
            p101b();
    
            p102a();
    
            p104a();
    
            p105a();
    
            p106a();
    
            p108a();
    
            p109a();
            p109b();
    
            p110a();
    
            p111a();
        };
        
        let functions = vec![
            ("p86a", p86a as fn()),
    
            ("p87a", p87a as fn()),
    
            ("p88a", p88a as fn()),
            ("p88b", p88b as fn()),
            ("p88c", p88c as fn()),
    
            ("p92a", p92a as fn()),
    
            ("p94a", p94a as fn()),
            ("p94b", p94b as fn()),
    
            ("p95a", p95a as fn()),
            ("p95b", p95b as fn()),
    
            ("p96a", p96a as fn()),
    
            ("p97a", p97a as fn()),
    
            ("p98a", p98a as fn()),
    
            ("p99a", p99a as fn()),
            ("p99b", p99b as fn()),
    
            ("p100a", p100a as fn()),
            ("p100b", p100b as fn()),
    
            ("p101a", p101a as fn()),
            ("p101b", p101b as fn()),
    
            ("p102a", p102a as fn()),
    
            ("p104a", p104a as fn()),
    
            ("p105a", p105a as fn()),
    
            ("p106a", p106a as fn()),
    
            ("p108a", p108a as fn()),
    
            ("p109a", p109a as fn()),
            ("p109b", p109b as fn()),
    
            ("p110a", p110a as fn()),
    
            ("p111a", p111a as fn()),
        ];
    
        done_and_show_used_milliseconds(main_function_name, action);
        done_and_show_used_seconds(main_function_name, action);
    
        done_and_show_used_milliseconds_for_vec(functions.clone());
        done_and_show_used_seconds_for_vec(functions.clone());

        println!();
        println!("  end: {:?}", Instant::now());
    });
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!("Calling {name} tooks {:?} milliseconds.", now.elapsed().as_millis());
}

#[allow(dead_code)]
fn done_and_show_used_seconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!("Calling {name} tooks {:?} seconds.", now.elapsed().as_secs());
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!("Calling {name} tooks {:?} milliseconds.", now.elapsed().as_millis());

    }
    println!("Total used {:?} milliseconds.", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn done_and_show_used_seconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!("Calling {name} tooks {:?} seconds.", now.elapsed().as_secs());

    }
    println!("Total used {:?} seconds.", start.elapsed().as_secs());
}

/// ```
/// let integer_i8: i8 = 0;
/// println!("The type of {integer_i8} is {}.", type_of(&integer_i8));
/// let integer_i16: i16 = 0;
/// println!("The type of {integer_i16} is {}.", type_of(&integer_i16));
/// let integer_i32: i32 = 0;
/// println!("The type of {integer_i32} is {}.", type_of(&integer_i32));
/// let integer_i64: i64 = 0;
/// println!("The type of {integer_i64} is {}.", type_of(&integer_i64));
/// let integer_i128: i128 = 0;
/// println!("The type of {integer_i128} is {}.", type_of(&integer_i128));
/// let integer_isize: isize = 0;
/// println!(
///     "The type of {integer_isize} is {}.",
///     type_of(&integer_isize)
/// );
/// println!();
/// let integer_u8: u8 = 0;
/// println!("The type of {integer_u8} is {}.", type_of(&integer_u8));
/// let integer_u16: u16 = 0;
/// println!("The type of {integer_u16} is {}.", type_of(&integer_u16));
/// let integer_u32: u32 = 0;
/// println!("The type of {integer_u32} is {}.", type_of(&integer_u32));
/// let integer_u64: u64 = 0;
/// println!("The type of {integer_u64} is {}.", type_of(&integer_u64));
/// let integer_u128: u128 = 0;
/// println!("The type of {integer_u128} is {}.", type_of(&integer_u128));
/// let integer_usize: usize = 0;
/// println!(
///     "The type of {integer_usize} is {}.",
///     type_of(&integer_usize)
/// );
/// println!();
/// 
/// let float_f32: f32 = 0.0;
/// println!("The type of 0.0 is {}.", type_of(&float_f32));
/// let float_f64: f64 = 0.0;
/// println!("The type of 0.0 is {}.", type_of(&float_f64));
/// 
/// let bool_true = true;
/// println!("The type of {bool_true} is {}.", type_of(&bool_true));
/// let bool_false = false;
/// println!("The type of {bool_false} is {}.", type_of(&bool_false));
/// println!();
/// 
/// let char: char = 'A';
/// println!("The type of 'A' is {}.", type_of(&char));
/// println!();
/// 
/// // String &str
/// let s = String::from("hello");
/// println!("The type of String::from(\"hello\") is {}.", type_of(&s));
/// let str = "hello";
/// println!("The type of \"hello\" is {}.", type_of(&str));
/// println!();
/// 
/// // Vec array tuple
/// let vec = vec![1, 2];
/// println!("The type of {vec:?} is {}.", type_of(&vec));
/// let array = [1, 2];
/// println!("The type of {array:?} is {}.", type_of(&array));
/// let tuple = (1, "test");
/// println!("The type of {tuple:?} is {}.", type_of(&tuple));
/// println!();
/// 
/// // closure
/// let action = || {
///     println!("call action()");
/// };
/// // println!("The type of {action} is {}.", type_of(&action));
/// // println!("The type of {action:?} is {}.", type_of(&action));
/// // println!("The type of {action:#?} is {}.", type_of(&action));
/// println!("The type of action is {}.", type_of(&action));
/// println!();
/// ```
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}