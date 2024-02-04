// 146.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/concurrency/dining-philosophers-async.html

// See: example_146\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

/*
The dining philosophers problem is a classic problem in concurrency:

Five philosophers dine together at the same table. Each philosopher has their own place at the table. 
There is a fork between each plate. The dish served is a kind of spaghetti which has to be eaten with two forks. 
Each philosopher can only alternately think and eat. Moreover, a philosopher can only eat their spaghetti 
when they have both a left and right fork. Thus two forks will only be available when their two nearest neighbors are thinking,
not eating. After an individual philosopher finishes eating, they will put down both forks.
*/

use std::sync::Arc;
use tokio::time;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;

struct Fork;

struct Philosopher {
    name: String,
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name)).await
            .unwrap();
    }

    async fn eat(&self) {
        // Pick up forks...
        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

#[tokio::main]
async fn main() {
    // Create forks

    // Create philosophers

    // Make them think and eat

    // Output their thoughts
}

/* result:

*/
