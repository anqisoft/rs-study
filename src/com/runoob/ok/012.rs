// 012.rs
// https://www.runoob.com/rust/rust-loop.html

fn main() {
    println!("Test while loop:");
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT, number is {}", number);

    println!("Test while loop again:");
    let mut k = 0;
    while k < 10 {
        println!("In while loop, i is {}", k);
        k += 1;
    }
    println!("EXIT while, i is {}", k);

    println!("");
    println!("Test for loop:");
    let a = [10, 20, 30, 40, 50];
    for j in a.iter() {
        println!("{}", j);
    }
    // error[E0425]: cannot find value `j` in this scope
    // println!("EXIT for, j is {}", j);
    println!("EXIT for",);

    println!("");
    println!("Test for loop again:");
    for k in 0..5 {
        println!("a[{}] = {}", k, a[k]);
    }
    println!("EXIT for, k is {}", k);
}

/* result:
Test while loop:
1
2
3
EXIT, number is 4
Test while loop again:
In while loop, i is 0
In while loop, i is 1
In while loop, i is 2
In while loop, i is 3
In while loop, i is 4
In while loop, i is 5
In while loop, i is 6
In while loop, i is 7
In while loop, i is 8
In while loop, i is 9
EXIT while, i is 10

Test for loop:
10
20
30
40
50
EXIT for

Test for loop again:
a[0] = 10
a[1] = 20
a[2] = 30
a[3] = 40
a[4] = 50
EXIT for, k is 10 -- Why not 4 or 5?
*/
