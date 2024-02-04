// eg15-2
// #[allow(dead_code)]
// enum List {
//     Cons(i32, List),
//     Nil,
// }
/* VsCode
  recursive type `List` has infinite sizerustcClick for full compiler diagnostic
  eg02.rs(4, 15): recursive without indirection
  eg02.rs(4, 15): insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle: `Box<`, `>`
*/
