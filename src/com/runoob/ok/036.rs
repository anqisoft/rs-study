// 036.rs
// https://www.runoob.com/rust/rust-project-management.html

mod nation {
    pub mod government {
        pub fn govern() {
            println!("nation::government::govern()");
        }
    }

    mod congress {
        pub fn legislate() {
            println!("nation::congress::legislate()");
        }
    }

    // pub(crate) mod court { // Same to
    pub mod court {
        // pub(crate) fn judicial() { // Same to
        pub fn judicial() {
            super::congress::legislate();
        }
    }
}

fn main() {
    nation::government::govern();
    nation::court::judicial();
}

/* result:
nation::government::govern()
nation::congress::legislate()
*/
