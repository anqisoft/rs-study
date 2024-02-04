// 030.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/static-and-const.html

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");

    // Option<u8>` cannot be formatted with the default formatter
    // println!("ZERO: {ZERO}");
    println!("ZERO: {ZERO:?}");
}

/* result:
digest: [222, 254, 150]
ZERO: Some(42)
*/
