fn p215a() {
    println!("\nIn p215a(), eg10-11");
    mod fix_eg10_11 {
        #[derive(Debug, Copy, Clone)]
        pub struct Point<X1, Y1> {
            pub x: X1,
            pub y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            pub  fn mixup<X2, Y2>(&self, other: &Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
    }

    use fix_eg10_11::Point as FixedPoint;
    let p1 = FixedPoint { x: 5, y: 10.4 };
    let p2 = FixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(&p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p1: {p1:?}, p2: {p2:?}, p3: {p3:?}");
}
