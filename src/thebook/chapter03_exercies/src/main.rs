/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * src\thebook\chapter3_exercies\main.rs
 *
 * <en>
 * Created on Fri Nov 24 2023 09:02:07
 * Feature: Test the exercises at the end of Chapter 3:
 * 1) Convert temperatures between Celsius and Fahrenheit,
 * 2) Generate the nth Fibonacci number,
 * 3) Print the lyrics to the Christmas carol "The Twelve Days of Christmas" and use the repeated parts in the song (write a loop).
 * </en>
 *
 * <cn>
 * 创建：2023年11月24日 09:02:07
 * 功能：测试第三章末尾练习题：
 * 相互转换摄氏与华氏温度
 * 生成第n个斐波那契数
 * 打印圣诞颂歌”The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。
 * </cn>
 *
 * <tw>
 * 創建：2023年11月24日 09:02:07
 * 功能：測試第三章末尾練習題：
 * 1) 相互轉換攝氏與華氏溫度
 * 2) 生成第n個斐波那契數
 * 3) 打印聖誕頌歌”The Twelve Days of Christmas” 的歌詞，並利用歌曲中的重複部分（編寫循環）。
 * </tw>
 */

use std::time::Instant;

/// <en>
/// Generate the nth Fibonacci number:
/// The first two items are both 1, and the index starts from zero.
/// </en>
/// <cn>
/// 生成第n个斐波那契数：前两项都是1，索引从零开始。
/// </cn>
/// <tw>
/// 生成第n個斐波那契數：前兩項都是1，索引從零開始。
/// </tw>
#[allow(dead_code)]
fn fibonacci(n: u16) -> u128 {
    // u128 >= 0
    // if n < 0 { panic!("n must be greater than or equal to zero."); }

    if n < 2 {
        return 1;
    }

    fibonacci(n - 2) + fibonacci(n - 1)
}

/// <en>
/// Use "match" to generate the nth Fibonacci number:
/// The first two items are both 1, and the index starts from zero.
/// </en>
/// <cn>
/// 使用“match”生成第n个斐波那契数：前两项都是1，索引从零开始。
/// </cn>
/// <tw>
/// 使用「match」產生第n個斐波那契數：前兩項都是1，索引從零開始。
/// </tw>
fn fibonacci_use_match(n: u16) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_use_match(n - 1) + fibonacci_use_match(n - 2),
    }
}

// See: https://www.cnblogs.com/wujuncheng/archive/2023/11/16/17835814.html

/// <en>
/// Use loop to generate the nth Fibonacci number:
/// The first two items are both 1, and the index starts from zero.
/// </en>
/// <cn>
/// 使用loop生成第n个斐波那契数：前两项都是1，索引从零开始。
/// </cn>
/// <tw>
/// 使用loop產生第n個斐波那契數：前兩項都是1，索引從零開始。
/// </tw>
fn fibonacci_use_loop(n: u16) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        _ => {
            let mut second_item_before: u128 = 1;
            let mut previous_item: u128 = 1;
            let mut v: u128 = 0;
            for _i in 2..n {
                v = second_item_before + previous_item;

                second_item_before = previous_item;
                previous_item = v;
            }
            v
        }
    }
}

/// <en>
/// Use vec to generate the nth Fibonacci number:
/// The first two items are both 1, and the index starts from zero.
/// </en>
/// <cn>
/// 使用vec生成第n个斐波那契数：前两项都是1，索引从零开始。
/// </cn>
/// <tw>
/// 使用vec產生第n個斐波那契數：前兩項都是1，索引從零開始。
/// </tw>
fn fibonacci_by_vec(n: u16) {
    println!("\nIn fibonacci_by_vec()");
    // https://www.cnblogs.com/Nazorine/p/16707498.html

    // https://rustwiki.org/zh-CN/std/vec/struct.Vec.html
    // https://blog.csdn.net/qq_39308071/article/details/114063758

    let now = Instant::now();
    let mut datas: Vec<u128> = Vec::with_capacity(n as usize);
    // // Error
    // for i in 0..n {
    //     if i < 2 {
    //         datas[i as usize] = 1;
    //     }
    //     else {
    //         datas[i as usize] = datas[(i - 2) as usize] + datas[(i - 1) as usize];
    //     }
    // }

    // Ok
    for i in 0..n {
        if i < 2 {
            datas.push(1);
        } else {
            datas.push(datas[(i - 2) as usize] + datas[(i - 1) as usize]);
        }
    }
    for i in 0..n {
        println!("fibonacci[{i}] is {}", datas[i as usize]);
    }
    println!(
        "Used {:?} milliseconds by pushing data.",
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    // ok
    let mut datas: Vec<u128> = vec![0; n as usize];
    for i in 0..n {
        if i < 2 {
            datas[i as usize] = 1;
        } else {
            datas[i as usize] = datas[(i - 2) as usize] + datas[(i - 1) as usize];
        }
    }
    for i in 0..n {
        println!("fibonacci[{i}] is {}", datas[i as usize]);
    }
    println!(
        "Used {:?} milliseconds by direct assignment.",
        now.elapsed().as_millis()
    );
}

/// <en>
/// Use array to generate the nth Fibonacci number:
/// The first two items are both 1, and the index starts from zero.
/// </en>
/// <cn>
/// 使用数组生成第n个斐波那契数：前两项都是1，索引从零开始。
/// </cn>
/// <tw>
/// 使用陣列產生第n個斐波那契數：前兩項都是1，索引從零開始。
/// </tw>
fn fibonacci_by_array(n: u16) {
    println!("\nIn fibonacci_by_array()");
    // https://www.cnblogs.com/Nazorine/p/16707498.html

    const FIBONACCI_MAX_COUNT: usize = 186;

    // attempt to use a non-constant value in a constant, this would need to be a `const`
    // let mut datas: [u128; n] = [0; n];

    let now = Instant::now();
    let mut datas: [u128; FIBONACCI_MAX_COUNT] = [0; FIBONACCI_MAX_COUNT];
    for i in 0..n {
        if i < 2 {
            datas[i as usize] = 1;
        } else {
            datas[i as usize] = datas[(i - 2) as usize] + datas[(i - 1) as usize];
        }
    }
    for i in 0..n {
        println!("fibonacci[{i}] is {}", datas[i as usize]);
    }
    println!("Used {:?} milliseconds.", now.elapsed().as_millis());
}

/// <en>
/// Convert temperatures between Celsius, Fahrenheit, and Kelvin.
/// When the endings are C, F and K respectively, it means that the original temperature is Celsius, Fahrenheit
/// and Kelvin respectively, and the corresponding three temperature values will be displayed.
/// </en>
/// <cn>
/// 相互转换摄氏温度、华氏温度、开氏温度。
/// 当结尾分别是C、F和K时，分别表示原始温度为摄氏温度、华氏温度、开氏温度，将显示其对应的三种温度值。
/// </cn>
/// <tw>相互轉換攝氏溫度、華氏溫度、開氏溫度。
/// 當結尾分別是C、F和K時，分別表示原始溫度為攝氏溫度、華氏溫度、開氏溫度，將顯示其對應的三種溫度值。
/// </tw>
fn convert_temperatures(original: &mut String) {
    // enum OriginalTemperatureKind {
    //     Celsius,
    //     Fahrenheit,
    //     Kelvin,
    // }

    const CELSIUS_TO_KELVIN: f32 = 273.15;

    *original = original.to_uppercase();
    let original_with_unit = original.clone();

    let origial_temperature_value = original
        .replace("C", "")
        .replace("F", "")
        .replace("K", "")
        .trim()
        .parse::<f32>()
        .expect("Not number");

    let celsius: f32;
    let fahrenheit: f32;
    let kelvin: f32;

    // https://wap.peopleapp.com/article/rmh22646635/rmh22646635
    if original.ends_with("F") {
        fahrenheit = origial_temperature_value;

        celsius = 5.0 * (fahrenheit - 32.0) / 9.0;
        kelvin = celsius + 273.15;
    } else if original.ends_with("K") {
        kelvin = origial_temperature_value;

        celsius = kelvin - CELSIUS_TO_KELVIN;
        fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    } else {
        // if original.ends_with("C") {
        celsius = origial_temperature_value;

        kelvin = celsius + CELSIUS_TO_KELVIN;
        fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    }

    println!("{original_with_unit} is {celsius}℃, and {fahrenheit}℉, and {kelvin}K.");
}

/*
// https://www.classicfm.com/discover-music/occasions/christmas/twelve-12-days-of-christmas-lyrics-meaning/
On the first day of Christmas
My true love gave to me
A partridge in a pear tree.

On the second day of Christmas
My true love gave to me
Two turtle doves
And a partridge in a pear tree.

On the third day of Christmas
My true love gave to me
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the fourth day of Christmas
My true love gave to me
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the fifth day of Christmas
My true love gave to me
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the sixth day of Christmas
My true love gave to me
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the seventh day of Christmas
My true love gave to me
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the eighth day of Christmas
My true love gave to me
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the ninth day of Christmas
My true love gave to me
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the tenth day of Christmas
My true love gave to me
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the eleventh day of Christmas
My true love gave to me
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.

On the twelfth day of Christmas
My true love gave to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves
And a partridge in a pear tree.
*/
/// <en>
/// Print the lyrics to the Christmas carol "The Twelve Days of Christmas"
/// and use the repeated parts in the song (write a loop).
/// </en>
/// <cn>
/// 打印圣诞颂歌”The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。
/// </cn>
/// <tw>
/// 打印聖誕頌歌”The Twelve Days of Christmas” 的歌詞，並利用歌曲中的重複部分（編寫循環）。
/// </tw>
fn show_the_lyrics_of_the_twelve_days_of_christmas() {
    println!("\nCall show_the_lyrics_of_the_twelve_days_of_christmas()");

    let no_and_lyrics = [
        // Remove the first two chars.
        ("first", "partridge in a pear tree."),
        ("second", "Two turtle doves,"),
        ("third", "Three French hens,"),
        ("fourth", "Four calling birds,"),
        ("fifth", "Five golden rings,"),
        ("sixth", "Six geese a-laying,"),
        ("seventh", "Seven swans a-swimming,"),
        ("eighth", "Eight maids a-milking,"),
        ("ninth", "Nine ladies dancing,"),
        ("tenth", "Ten lords a-leaping,"),
        ("eleventh", "Eleven pipers piping,"),
        ("twelfth", "Twelve drummers drumming,"),
    ];
    let no_and_lyric_count = no_and_lyrics.len();

    for outer_index in 0..no_and_lyric_count {
        println!("On the {} day of Christmas,", no_and_lyrics[outer_index].0);
        println!("My true love gave to me,");

        for inner_index in (0..(outer_index + 1)).rev() {
            let inner = no_and_lyrics[inner_index];
            let lyric = inner.1;
            if inner_index == 0 {
                println!(
                    "{} {lyric}",
                    if outer_index == inner_index {
                        "A"
                    } else {
                        "And a"
                    }
                );
            } else {
                println!("{lyric}");
            }
        }

        println!();
    }
}

fn main() {
    const FIBONACCI_ELEMENTS_COUNT: u16 = 40;
    const FIBONACCI_ELEMENTS_MAX_COUNT: u16 = FIBONACCI_ELEMENTS_COUNT * 4 + 26;

    // fibonacci
    println!("\nCall fibonacci()");
    let now = Instant::now();
    for n in 0..40 {
        println!("fibonacci({n}) is {}", fibonacci(n));
    }
    println!(
        "Used {:?} milliseconds for calculating the 40-term Fibolina sequence recursively.",
        now.elapsed().as_millis()
    );
    for n in 40..50 {
        let now = Instant::now();
        println!("fibonacci({n}) is {}, and used {:?} milliseconds.", fibonacci(n), now.elapsed().as_millis());
    }

    // fibonacci_use_match
    println!("\nCall fibonacci_use_match()");
    let now = Instant::now();
    for n in 0..40 {
        println!("fibonacci_use_match({n}) is {}", fibonacci_use_match(n));
    }
    println!(
        "Used {:?} milliseconds for calculating the 40-term Fibolina sequence recursively.",
        now.elapsed().as_millis()
    );
    for n in 40..50 {
        let now = Instant::now();
        println!("fibonacci_use_match({n}) is {}, and used {:?} milliseconds.", fibonacci_use_match(n), now.elapsed().as_millis());
    }

    // fibonacci_use_loop
    println!("\nCall fibonacci_use_loop()");
    let now = Instant::now();
    for n in 0..40 {
        println!("fibonacci_use_loop({n}) is {}", fibonacci_use_loop(n));
    }
    println!(
        "Used {:?} milliseconds for calculating the 40-term Fibolina sequence recursively.",
        now.elapsed().as_millis()
    );
    for n in 40..50 {
        let now = Instant::now();
        println!("fibonacci_use_loop({n}) is {}, and used {:?} milliseconds.", fibonacci_use_loop(n), now.elapsed().as_millis());
    }

    // fibonacci_by_vec(200) => thread 'main' panicked at src\main.rs:127:33: attempt to add with overflow
    // 180 ok, 190 panic, 185 ok, 188 panic, 187 panic, 186 ok. The maxium parameter is 186.
    fibonacci_by_vec(FIBONACCI_ELEMENTS_MAX_COUNT);

    fibonacci_by_array(FIBONACCI_ELEMENTS_MAX_COUNT);

    let now = Instant::now();
    println!("\nCall convert_temperatures()");
    for temperatures in ["0C", "0F", "0K", "-273.15C", "32F", "212F", "273.15K"] {
        convert_temperatures(&mut temperatures.to_string());
    }
    println!("Used {:?} milliseconds.", now.elapsed().as_millis());

    let now = Instant::now();
    show_the_lyrics_of_the_twelve_days_of_christmas();
    println!("Used {:?} milliseconds.", now.elapsed().as_millis());
}
