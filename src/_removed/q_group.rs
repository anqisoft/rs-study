eg231201a() {
  fn foo(input: &[String]) -> Vec<String> {
    input.to_vec()
  }
  
  let array = ["A".to_string(), "B".to_string()];
  let vec = foo(&array);
  println!("array is {array:?}");
  println!("vec is {vec:?}");
}