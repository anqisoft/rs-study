// 072.rs
// https://www.runoob.com/rust/rust-collection-string.html

fn main() {
    let string = String::new();
    let one = 1.to_string();
    let float = 1.3.to_string();
    let slice = "slice".to_string();
    println!("string is {}, one is {}, float is {}, slice is {}, ", string,one, float, slice );

    let hello = String::from("السلام عليكم"); println!("hello is {}", hello);
    let hello = String::from("Dobrý den");println!("hello is {}", hello);
    let hello = String::from("Hello");println!("hello is {}", hello);
    let hello = String::from("שָׁלוֹם");println!("hello is {}", hello);
    let hello = String::from("नमस्ते");println!("hello is {}", hello);
    let hello = String::from("こんにちは");println!("hello is {}", hello);
    let hello = String::from("안녕하세요");println!("hello is {}", hello);
    let hello = String::from("你好");println!("hello is {}", hello);
    let hello = String::from("Olá");println!("hello is {}", hello);
    let hello = String::from("Здравствуйте");println!("hello is {}", hello);
    let hello = String::from("Hola");println!("hello is {}", hello);

    let mut s = String::from("run");
    s.push_str("oob");
    s.push('!');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    let s = "你好";
    let len = s.len();
    println!("s is \"{}\", len is {}", s, len);

    let s = "hello你好";
    let len = s.chars().count();
    println!("s is \"{}\", len is {}", s, len);


    let s = String::from("hello中文");
    for c in s.chars() {
        println!("{}", c);
    }

    let s = String::from("hello中文");
    for c in s.chars() {
        println!("{}", c);
    }

    let s = String::from("EN中文");
    let sub = &s[0..2];
    println!("{}", sub);


    // let s = String::from("EN中文");
    // let sub = &s[0..3]; // byte index 3 is not a char boundary; it is inside '中' (bytes 2..5) of `EN中文`
    // println!("{}", sub);
}

/* result:
string is , one is 1, float is 1.3, slice is slice,
hello is السلام عليكم
hello is Dobrý den
hello is Hello
hello is שָׁלוֹם
hello is नमस्ते
hello is こんにちは
hello is 안녕하세요
hello is 你好
hello is Olá
hello is Здравствуйте
hello is Hola
s is runoob!
s3 is Hello, world!
s is tic-tac-toe
s is tic-tac-toe
s is "你好", len is 6
s is "hello你好", len is 7
h
e
l
l
o
中
文
h
e
l
l
o
中
文
EN
*/
