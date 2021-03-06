//! Polylog
//! =======
//!
//! The Polylog package provides Rust implementations of real and
//! complex polylogarithms.
//!
//! # Example:
//! ```
//! extern crate num;
//! extern crate polylog;
//! use num::complex::Complex;
//! use polylog::{Li2, Li3, Li4, Li5, Li6};
//!
//! fn main() {
//!     let x = 1.0;
//!     let z = Complex::new(1.0, 1.0);
//!     println!("Li2({}) = {}", x, x.li2());
//!     println!("Li2({}) = {}", z, z.li2());
//!     println!("Li3({}) = {}", z, z.li3());
//!     println!("Li4({}) = {}", z, z.li4());
//!     println!("Li5({}) = {}", z, z.li5());
//!     println!("Li6({}) = {}", z, z.li6());
//! }
//! ```


extern crate num;

mod cln;
mod li2;
mod li3;
mod li4;
mod li5;
mod li6;

pub use self::li2::Li2;
pub use self::li3::Li3;
pub use self::li4::Li4;
pub use self::li5::Li5;
pub use self::li6::Li6;
