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

fn main() {
    let integer_i8: i8 = 0;
    println!("The type of {integer_i8} is {}.", type_of(&integer_i8));
    let integer_i16: i16 = 0;
    println!("The type of {integer_i16} is {}.", type_of(&integer_i16));
    let integer_i32: i32 = 0;
    println!("The type of {integer_i32} is {}.", type_of(&integer_i32));
    let integer_i64: i64 = 0;
    println!("The type of {integer_i64} is {}.", type_of(&integer_i64));
    let integer_i128: i128 = 0;
    println!("The type of {integer_i128} is {}.", type_of(&integer_i128));
    let integer_isize: isize = 0;
    println!(
        "The type of {integer_isize} is {}.",
        type_of(&integer_isize)
    );
    println!();
    let integer_u8: u8 = 0;
    println!("The type of {integer_u8} is {}.", type_of(&integer_u8));
    let integer_u16: u16 = 0;
    println!("The type of {integer_u16} is {}.", type_of(&integer_u16));
    let integer_u32: u32 = 0;
    println!("The type of {integer_u32} is {}.", type_of(&integer_u32));
    let integer_u64: u64 = 0;
    println!("The type of {integer_u64} is {}.", type_of(&integer_u64));
    let integer_u128: u128 = 0;
    println!("The type of {integer_u128} is {}.", type_of(&integer_u128));
    let integer_usize: usize = 0;
    println!(
        "The type of {integer_usize} is {}.",
        type_of(&integer_usize)
    );
    println!();

    let float_f32: f32 = 0.0;
    println!("The type of 0.0 is {}.", type_of(&float_f32));
    let float_f64: f64 = 0.0;
    println!("The type of 0.0 is {}.", type_of(&float_f64));

    let bool_true = true;
    println!("The type of {bool_true} is {}.", type_of(&bool_true));
    let bool_false = false;
    println!("The type of {bool_false} is {}.", type_of(&bool_false));
    println!();

    let char: char = 'A';
    println!("The type of 'A' is {}.", type_of(&char));
    println!();

    // String &str
    let s = String::from("hello");
    println!("The type of String::from(\"hello\") is {}.", type_of(&s));
    let str = "hello";
    println!("The type of \"hello\" is {}.", type_of(&str));
    println!();

    // Vec array tuple
    let vec = vec![1, 2];
    println!("The type of {vec:?} is {}.", type_of(&vec));
    let array = [1, 2];
    println!("The type of {array:?} is {}.", type_of(&array));
    let tuple = (1, "test");
    println!("The type of {tuple:?} is {}.", type_of(&tuple));
    println!();

    // closure
    let action = || {
        println!("call action()");
    };
    // println!("The type of {action} is {}.", type_of(&action));
    // println!("The type of {action:?} is {}.", type_of(&action));
    // println!("The type of {action:#?} is {}.", type_of(&action));
    println!("The type of action is {}.", type_of(&action));
    println!();

    const one: u8 = 1;
    println!("The type of 1 is {}", type_of(&one));

    static two: f32 = 2.0;
    println!("The type of 2.0 is {}", type_of(&two));
}
/*
The type of 0 is i8.
The type of 0 is i16.
The type of 0 is i32.
The type of 0 is i64.
The type of 0 is i128.
The type of 0 is isize.

The type of 0 is u8.
The type of 0 is u16.
The type of 0 is u32.
The type of 0 is u64.
The type of 0 is u128.
The type of 0 is usize.

The type of 0.0 is f32.
The type of 0.0 is f64.
The type of true is bool.
The type of false is bool.

The type of 'A' is char.

The type of String::from("hello") is alloc::string::String.
The type of "hello" is &str.

The type of [1, 2] is alloc::vec::Vec<i32>.
The type of [1, 2] is [i32; 2].
The type of (1, "test") is (i32, &str).

The type of action is type_of::main::{{closure}}.
*/
