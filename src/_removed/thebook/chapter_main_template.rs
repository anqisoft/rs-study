/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter00\src\main.rs
 * 
 * https:∕∕kaisery.github.io∕trpl-zh-cn∕
 * https://kaisery.github.io/trpl-zh-cn/Rust%20%E7%A8%8B%E5%BA%8F%E8%AE%BE%E8%AE%A1%E8%AF%AD%E8%A8%80%20%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87%E7%89%88.pdf
 * 
 * https://doc.rust-lang.org/stable/book/
 * https://doc.rust-lang.org/book/
 *
 * <en>
 * Created on Sun Nov 26 2023 13:57:30
 * Feature: Test the code of Chapter 00 in the Chinese version pdf. 
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z). 
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年11月26日 13:57:30
 * 功能：测试中文版pdf中第00章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年11月26日 13:57:30
 * 功能：測試中文版pdf中第00章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
  let main_function_name = "chapter00_main";
  let start_chapter_line = "Chapter 00";
  done_and_show_used_milliseconds(main_function_name, || {
      println!("start: {:?}", Instant::now());
      let action = || {
          println!("{start_chapter_line}");
      
          //
      };
      
      let functions = vec![
          //
      ];
  
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

#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
  std::any::type_name::<T>()
}