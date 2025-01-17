// Copyright (c) 2020 Tomek Czajka
//
// Licensed under either of
//
// * Apache License, Version 2.0
//   (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
// * MIT license
//   (LICENSE-MIT or https://opensource.org/licenses/MIT)
//
// at your option.
//
// Unless you explicitly state otherwise, any contribution intentionally submitted
// for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
// dual licensed as above, without any additional terms or conditions.

//! A big integer library with good performance.
//!
//! The library implements efficient large integer arithmetic in pure Rust.
//!
//! The two main integer types are [UBig] (for unsigned integers) and [IBig] (for signed integers).
//!
//! Modular arithmetic is supported by the module [modular].
//!
//! # Examples
//!
//! ```
//! # use ibig::error::ParseError;
//! use ibig::{ibig, modular::ModuloRing, ubig, UBig};
//!
//! let a = ubig!(12345678);
//! let b = ubig!(0x10ff);
//! let c = ibig!(-azz base 36);
//! let d: UBig = "15033211231241234523452345345787".parse()?;
//!
//! assert_eq!(c.to_string(), "-14255");
//! assert_eq!(
//!     (a * b.pow(10)).in_radix(16).to_string(),
//!     "1589bda8effbfc495d8d73c83d8b27f94954e"
//! );
//! assert_eq!(
//!     format!("hello {:#x}", d % ubig!(0xabcd1234134132451345)),
//!     "hello 0x1a7e7c487267d2658a93"
//! );
//!
//! let ring = ModuloRing::new(&ubig!(10000));
//! let x = ring.from(12345);
//! let y = ring.from(55443);
//! assert_eq!(format!("{}", x - y), "6902 (mod 10000)");
//! # Ok::<(), ParseError>(())
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::{ibig::IBig, ubig::UBig};

mod add;
mod add_ops;
mod arch;
mod bits;
mod buffer;
mod cmp;
mod convert;
mod div;
mod div_ops;
pub mod error;
mod fast_divide;
pub mod fmt;
mod ibig;
mod math;
mod memory;
pub mod modular;
mod mul;
mod mul_ops;
pub mod ops;
mod parse;
mod pow;
mod primitive;
mod radix;
mod shift;
mod shift_ops;
mod sign;
mod ubig;

#[macro_use]
mod macros;

#[cfg(feature = "rand")]
pub mod rand;

#[cfg(feature = "num-traits")]
mod num_traits;
