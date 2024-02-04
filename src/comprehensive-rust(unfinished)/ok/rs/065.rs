// 065.rs
// https://google.github.io/comprehensive-rust/zh-CN/structs/tuple-structs.html

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    
    // expected `Newtons`, found `PoundsOfForce`
    // set_thruster_force(force);

    // an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    // set_thruster_force(force as Newtons);

    // help: a field with a similar name exists: `0`
    // set_thruster_force(force.p0);

    // expected `Newtons`, found `f64`
    // set_thruster_force(force.0);

    set_thruster_force(Newtons(force.0));
}

/* result:
warning: unused variable: `force`
  --> ..\rs\065.rs:11:23
   |
11 | fn set_thruster_force(force: Newtons) {
   |                       ^^^^^ help: if this is intentional, prefix it with an underscore: `_force`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

thread 'main' panicked at ..\rs\065.rs:8:5:
not yet implemented: Ask a rocket scientist at NASA
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
