use std::time::Instant;

/// <en>
/// Use tail recursion to generate the nth Fibonacci number:
/// The first two items are both 1, and the index starts from zero.
/// </en>
/// <cn>
/// 使用尾递归生成第n个斐波那契数：前两项都是1，索引从零开始。
/// </cn>
/// <tw>
/// 使用尾遞迴產生第n個斐波那契數：前兩項都是1，索引從零開始。
/// </tw>
fn fibonacci_use_tail_recursion(n: u16, second_item_before: u128, previous_item: u128) -> u128 {
  println!("call fibonacci_use_tail_recursion({n}, {second_item_before}, {previous_item})");
  match n {
      0 => 1,
      1 => 1,
      _ => fibonacci_use_tail_recursion(n - 1, previous_item, second_item_before + previous_item)
  }
}

fn main() {
  println!("\nCall fibonacci_use_tail_recursion()");
  let now = Instant::now();
  for n in 0..2 {
      println!("fibonacci_use_tail_recursion({n}) is {}\n", fibonacci_use_tail_recursion(n, 1, 1));
  }
  for n in 2..40 {
      println!("fibonacci_use_tail_recursion({n}) is {}\n", fibonacci_use_tail_recursion(n, fibonacci_use_tail_recursion(n - 2, 1, 1), fibonacci_use_tail_recursion(n - 1, 1, 1)));
  }
  println!(
      "Used {:?} milliseconds for calculating the 40-term Fibolina sequence recursively.",
      now.elapsed().as_millis()
  );

  for n in 40..50 {
      let now = Instant::now();
      // println!("fibonacci_use_tail_recursion({n}) is {}", fibonacci_use_tail_recursion(n, 1, 1));
      println!("fibonacci_use_tail_recursion({n}) is {}", fibonacci_use_tail_recursion(n, fibonacci_use_tail_recursion(n - 2, 1, 1), fibonacci_use_tail_recursion(n - 1, 1, 1)));
      println!(
          "Used {:?} milliseconds for calculating the {n}-term Fibolina sequence recursively.\n",
          now.elapsed().as_millis()
      );
  }
}