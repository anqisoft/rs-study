// 097.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/from-iterator.html

fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}

/* result:
prime_squares: [4, 9, 25, 49]
*/
