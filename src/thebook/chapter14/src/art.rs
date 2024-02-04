//! # Art
//!
//! A library for modeling artistic concepts.
//!
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    // /// Combines two primary colors in equal amounts to create a secondary color.
    // pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    //     if c1 == PrimaryColor::Red {
    //       if c2 == PrimaryColor::Yellow {
    //         return SecondaryColor::Orange;
    //       } else if c2 == PrimaryColor::Blue {
    //         return SecondaryColor::Purple;
    //       }
    //     }

    //     if c1 == PrimaryColor::Yellow {
    //       if c2 == PrimaryColor::Blue {
    //         return SecondaryColor::Green;
    //       }
    //     }
    // }
    // /* `if` expressions without `else` evaluate to `()` */
    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Yellow => Some(SecondaryColor::Orange),
                PrimaryColor::Blue => Some(SecondaryColor::Purple),
                _ => None,
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Blue => Some(SecondaryColor::Green),
                _ => None,
            },
            _ => None,
        }
    }
}
