/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter14\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch14-00-more-about-cargo.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch14-00-more-about-cargo.html</tw>
 *
 * <en>
 * Created on Fri Dec 07 2023 14:03:58
 * Feature: Test the code of Chapter 14 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年12月7日 14:03:58
 * 功能：测试中文版pdf中第14章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年12月7日 14:03:58
 * 功能：測試中文版pdf中第14章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

use chapter14::art;

/*
* <en>https://doc.rust-lang.org/book/ch14-01-release-profiles.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch14-01-release-profiles.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch14-01-release-profiles.html</tw>
*/

/*
* <en>https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch14-02-publishing-to-crates-io.html</tw>
*/

/*
* <en>https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch14-03-cargo-workspaces.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch14-03-cargo-workspaces.html</tw>
*/

/*
* <en>https://doc.rust-lang.org/book/ch14-04-installing-binaries.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch14-04-installing-binaries.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch14-04-installing-binaries.html</tw>
*/

/*
* <en>https://doc.rust-lang.org/book/ch14-05-extending-cargo.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch14-05-extending-cargo.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch14-05-extending-cargo.html</tw>
*/

fn p350a() {
    use art::kinds::PrimaryColor;
    use art::utils::mix;

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

fn p353a() {
    use art::mix;
    use art::PrimaryColor;

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter14_main";
    let start_chapter_line = "Chapter 14";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p350a();
            p353a();
        };

        let functions = vec![("p350a", p350a as fn()), ("p353a", p353a as fn())];

        done_and_show_used_milliseconds(main_function_name, action);
        // done_and_show_used_seconds(main_function_name, action);

        // done_and_show_used_milliseconds_for_vec(functions.clone());
        // done_and_show_used_seconds_for_vec(functions.clone());

        println!();
        println!("  end: {:?}", Instant::now());
    });
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!(
        "Calling {name} tooks {:?} milliseconds.",
        now.elapsed().as_millis()
    );
}

#[allow(dead_code)]
fn done_and_show_used_seconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!(
        "Calling {name} tooks {:?} seconds.",
        now.elapsed().as_secs()
    );
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!(
            "Calling {name} tooks {:?} milliseconds.",
            now.elapsed().as_millis()
        );
    }
    println!("Total used {:?} milliseconds.", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn done_and_show_used_seconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!(
            "Calling {name} tooks {:?} seconds.",
            now.elapsed().as_secs()
        );
    }
    println!("Total used {:?} seconds.", start.elapsed().as_secs());
}

#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
