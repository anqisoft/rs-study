// 041.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/if-let-expressions.html

fn main() {
    println!("{:?}", second_word_to_upper("foo bar"));
    println!("{:?}", third_word_to_upper("foo bar test"));
}
 
fn second_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    let (Some(_), Some(item)) = (it.next(), it.next()) else {
        return None;
    };
    Some(item.to_uppercase())
}
 
// fn third_word_to_upper(s: &str) -> Option<String> {
//     let mut it = s.split(' ');
//     // help: if this is intentional, prefix it with an underscore: `_item`
//     // let (Some(_), Some(item), Some(third)) = (it.next(), it.next(), it.next()) else {
//     let (Some(_), Some(_second), Some(third)) = (it.next(), it.next(), it.next()) else {
//         return None;
//     };
//     Some(third.to_uppercase())
// }
 
fn third_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    
    it.next();
    it.next();
    // warning: unnecessary parentheses around pattern
    // = note: `#[warn(unused_parens)]` on by default help: remove these parentheses
    // warning: unnecessary parentheses around assigned value
    // help: remove these parentheses
    // let (Some(third)) = (it.next()) else {
    let Some(third) = it.next() else {
        return None;
    };
    Some(third.to_uppercase())
}

/* result:
Some("BAR")
*/
