/// See: https://blog.51cto.com/xdataplus/5982929, and I changed it.
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
  // std::intrinsics::type_name::<T>()
}

// /// ```
// /// done_and_show_used_milliseconds(&|| {
// ///   println!("Chapter 04");
// ///
// ///   p86a();
// ///   p87a();
// ///   p88a();
// /// });
// /// ```
// fn done_and_show_used_milliseconds(func: &dyn Fn()) {
//   let now = Instant::now();
//   func();
//   println!("Used {:?} milliseconds.", now.elapsed().as_millis());
// }

// See: https://doc.rust-lang.org/reference/types/closure.html
/// ```
/// done_and_show_used_milliseconds(&|| {
///   println!("Chapter 04");
///
///   p86a();
///   p87a();
///   p88a();
/// });
/// ```
/// or
/// ```
/// done_and_show_used_milliseconds(|| {
///   println!("Chapter 04");
///
///   p86a();
///   p87a();
///   p88a();
/// });
/// ```
#[allow(dead_code)]
fn done_and_show_used_milliseconds(func: impl Fn()) {
  let now = Instant::now();
  func();
  println!("Used {:?} milliseconds.", now.elapsed().as_millis());
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