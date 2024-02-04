/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter${chapter}\src\main.rs
 *
 * Reference Links:
 * <en>https://doc.rust-lang.org/book/${page_name}.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/${page_name}.html</cn>
 * <tw>https://rust-lang.tw/book-tw/${page_name}.html</tw>
*/

/* <en>
 * Created on Thu Dec 14 2023 11:24:17
 * Feature: Test the code of chapter ${chapter}.
 * The method name starts with eg, followed by a serial number.
 * For example, eg1 represents the first example in this chapter.
 * </en>
*/

/* <cn>
 * 创建：2023年12月14日 11:24:17
 * 功能：测试第${chapter}章的代码。方法名以eg开头，后面加序号，如eg1表示本章第一个例子。
 * </cn>
*/

/* <tw>
 * 創建：2023年12月14日 11:24:17
 * 功能：測試第${chapter}章的程式碼。方法名稱以eg開頭，後面加序號，如eg1表示本章第一個例子。
 * </tw>
*/

fn eg09a() {
    // use gui::{Button, Screen};
    use chapter17::gui::{Button, Screen};

    // fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
    // }
}

fn eg09b() {
    // use gui::{Button, Screen};
    use chapter17::eg03::{Button, Screen};

    // fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
    // }
}

fn eg10() {
    // use gui::Screen;
    use chapter17::gui::Screen;

    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}

fn eg11() {
    // use blog::Post;
    use chapter17::blog::Post;

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn eg20() {
    // use blog::Post;
    use chapter17::blog_new::Post;

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // error
    // assert_eq!("", post.content());

    let post = post.request_review();
    // error
    // assert_eq!("", post.content());

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

fn main() {
    eg09a();
    eg09b();
    eg10();
    eg11();
    eg20();
}
