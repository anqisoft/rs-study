// 053.rs
// https://www.runoob.com/rust/rust-generics.html
trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}
struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name.clone(), self.age)
    }
}

struct Dog {
    kind: String,
}

impl Descriptive for Dog {
    fn describe(&self) -> String {
        format!("Dog's kind is {}.", self.kind)
    }
}

fn output1(object: impl Descriptive) {
    println!("{}", object.describe());
}

fn output2<T: Descriptive>(object: T) {
    println!("{}", object.describe());
}

fn output_two<T: Descriptive>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

trait Summary {
    fn summary(&self) -> String {
        // println!("summary()");
        String::from("summary()")
    }
}
trait Display {
    fn display(&self) -> String {
        // println!("display()");
        String::from( "display()")
    }
}
fn notify1(item: impl Summary + Display){
    println!("\nIn notify1(), \nsummary is {}, \ndisplay is {}", item.summary(), item.display());
}
fn notify2<T: Summary + Display>(item: T){
    println!("\nIn notify2(), \nsummary is {}, \ndisplay is {}", item.summary(), item.display());
}

struct Robot {
    name: String,
    voice: u8,
}

impl Summary for Robot {
    fn summary(&self) -> String {
        format!("summary(): name is {}", self.name.clone())
    }
}

impl Display for Robot {
    fn display(&self) -> String {
        format!("display(): voice is {}", self.voice)
    }
}

fn main() {
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());

    // <en>I once had a dog named Butterfly.</en>
    // <cn>我曾经养过一条狗，名字叫蝴蝶。</cn>
    // <tw>我曾经养过一条狗，名字叫蝴蝶。</tw>
    let butterfly = Dog {
        kind: String::from("The ears are like butterflies"),
    };
    output1(&cali);
    output1(butterfly);

    let judy = Person {
        name: String::from("Judy"),
        age: 21
    };
    output_two(cali, judy);

    let robot = Robot {
        name: String::from("xiaoai"),
        voice: 100,
    };
    notify1(robot);
    notify2(robot);

    fn person() -> impl Descriptive {
        Person {
            name: String::from("Cali"),
            age: 24
        }
    }

    fn some_function(bool bl) -> impl Descriptive {
        if bl {
            return A {};
        } else {
            return B {};
        }
    }

    struct A<T> {}

    impl<T: B + C> A<T> {
        fn d(&self) {}
    }
}

/* result:

*/
