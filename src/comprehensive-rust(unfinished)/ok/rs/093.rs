// 093.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/default-methods.html

fn test1() {
    println!("In test1()");
    
    trait Equals {
        fn equals(&self, other: &Self) -> bool;
        fn not_equals(&self, other: &Self) -> bool {
            !self.equals(other)
        }
    }

    #[derive(Debug)]
    struct Centimeter(i16);

    impl Equals for Centimeter {
        fn equals(&self, other: &Centimeter) -> bool {
            self.0 == other.0
        }
    }
    
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equals(&b));
}

fn test2() {
    println!("\nIn test2()");

    trait Equals {
        fn equals(&self, other: &Self) -> bool;
    }
    
    trait NotEquals: Equals {
        fn not_equals(&self, other: &Self) -> bool {
            !self.equals(other)
        }
    }
    
    #[derive(Debug)]
    struct Centimeter(i16);
    
    impl Equals for Centimeter {
        fn equals(&self, other: &Centimeter) -> bool {
            self.0 == other.0
        }
    }
    impl NotEquals for Centimeter { }
    
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equals(&b));
}

fn test3() {
    println!("\nIn test3()");
    trait Equals {
        fn equals(&self, other: &Self) -> bool;
    }
    
    trait NotEquals {
        fn not_equals(&self, other: &Self) -> bool;
    }
    
    #[derive(Debug)]
    struct Centimeter(i16);
    
    impl Equals for Centimeter {
        fn equals(&self, other: &Centimeter) -> bool {
            self.0 == other.0
        }
    }
    impl<T> NotEquals for T where T: Equals {
        fn not_equals(&self, other: &Self) -> bool {
            !self.equals(other)
        }
    }

    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equals(&b));    
}

fn main() {
    test1();
    test2();
    test3();
}

/* result:
In test1()
Centimeter(10) equals Centimeter(20): false
Centimeter(10) not_equals Centimeter(20): true

In test2()
Centimeter(10) equals Centimeter(20): false
Centimeter(10) not_equals Centimeter(20): true

In test3()
Centimeter(10) equals Centimeter(20): false
Centimeter(10) not_equals Centimeter(20): true
*/
