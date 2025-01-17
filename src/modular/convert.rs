//! Conversion between Modulo, UBig and IBig.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    div,
    ibig::IBig,
    memory::MemoryAllocation,
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
        modulo_ring::{ModuloRing, ModuloRingLarge, ModuloRingRepr, ModuloRingSmall},
    },
    primitive::extend_word,
    shift,
    sign::Sign::*,
    ubig::{Repr, UBig},
};
use alloc::vec::Vec;
use core::iter;

impl ModuloRing {
    /// The ring modulus.
    ///
    /// # Example
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// assert_eq!(ring.modulus(), ubig!(100));
    /// ```
    pub fn modulus(&self) -> UBig {
        match self.repr() {
            ModuloRingRepr::Small(self_small) => UBig::from_word(self_small.modulus()),
            ModuloRingRepr::Large(self_large) => self_large.modulus(),
        }
    }

    /// Create an element of the ring from another type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// let x = ring.from(-1234);
    /// let y = ring.from(ubig!(3366));
    /// assert!(x == y);
    /// ```
    pub fn from<T: IntoModulo>(&self, x: T) -> Modulo {
        x.into_modulo(self)
    }
}

impl ModuloRingSmall {
    pub(crate) fn modulus(&self) -> Word {
        self.normalized_modulus() >> self.shift()
    }

    pub(crate) const fn normalize_word(&self, word: Word) -> Word {
        if self.shift() == 0 {
            self.fast_div().div_rem_word(word).1
        } else {
            self.fast_div().div_rem(extend_word(word) << self.shift()).1
        }
    }

    pub(crate) fn normalize_large(&self, words: &[Word]) -> Word {
        let rem = div::fast_rem_by_normalized_word(words, *self.fast_div());
        if self.shift() == 0 {
            rem
        } else {
            self.fast_div().div_rem(extend_word(rem) << self.shift()).1
        }
    }
}

impl ModuloRingLarge {
    pub(crate) fn modulus(&self) -> UBig {
        let normalized_modulus = self.normalized_modulus();
        let mut buffer = Buffer::allocate(normalized_modulus.len());
        buffer.extend(normalized_modulus);
        let low_bits = shift::shr_in_place(&mut buffer, self.shift());
        assert!(low_bits == 0);
        buffer.into()
    }
}

impl Modulo<'_> {
    /// Get the residue in range `0..n` in an n-element ring.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// let x = ring.from(-1234);
    /// assert_eq!(x.residue(), ubig!(66));
    /// ```
    pub fn residue(&self) -> UBig {
        match self.repr() {
            ModuloRepr::Small(self_small) => UBig::from_word(self_small.residue()),
            ModuloRepr::Large(self_large) => self_large.residue(),
        }
    }
}

impl ModuloSmall<'_> {
    pub(crate) fn residue(&self) -> Word {
        self.normalized_value() >> self.ring().shift()
    }
}

impl ModuloLarge<'_> {
    pub(crate) fn residue(&self) -> UBig {
        let words = self.normalized_value();
        let mut buffer = Buffer::allocate(words.len());
        buffer.extend(words);
        let low_bits = shift::shr_in_place(&mut buffer, self.ring().shift());
        assert!(low_bits == 0);
        buffer.into()
    }
}

/// Trait for types that can be converted into [Modulo] in a [ModuloRing].
pub trait IntoModulo {
    fn into_modulo(self, ring: &ModuloRing) -> Modulo;
}

impl IntoModulo for UBig {
    fn into_modulo(self, ring: &ModuloRing) -> Modulo {
        match ring.repr() {
            ModuloRingRepr::Small(ring_small) => ModuloSmall::from_ubig(&self, ring_small).into(),
            ModuloRingRepr::Large(ring_large) => ModuloLarge::from_ubig(self, ring_large).into(),
        }
    }
}

impl IntoModulo for &UBig {
    fn into_modulo(self, ring: &ModuloRing) -> Modulo {
        match ring.repr() {
            ModuloRingRepr::Small(ring_small) => ModuloSmall::from_ubig(self, ring_small).into(),
            ModuloRingRepr::Large(ring_large) => {
                ModuloLarge::from_ubig(self.clone(), ring_large).into()
            }
        }
    }
}

impl IntoModulo for IBig {
    fn into_modulo(self, ring: &ModuloRing) -> Modulo {
        let (sign, mag) = self.into_sign_magnitude();
        let modulo = mag.into_modulo(ring);
        match sign {
            Positive => modulo,
            Negative => -modulo,
        }
    }
}

impl IntoModulo for &IBig {
    fn into_modulo(self, ring: &ModuloRing) -> Modulo {
        let modulo = self.magnitude().into_modulo(ring);
        match self.sign() {
            Positive => modulo,
            Negative => -modulo,
        }
    }
}

impl<'a> ModuloSmall<'a> {
    pub(crate) fn from_ubig(x: &UBig, ring: &'a ModuloRingSmall) -> ModuloSmall<'a> {
        let normalized_value = match x.repr() {
            Repr::Small(word) => ring.normalize_word(*word),
            Repr::Large(words) => ring.normalize_large(words),
        };
        ModuloSmall::new(normalized_value, ring)
    }
}

impl<'a> ModuloLarge<'a> {
    pub(crate) fn from_ubig(mut x: UBig, ring: &'a ModuloRingLarge) -> ModuloLarge<'a> {
        x <<= ring.shift() as usize;
        let modulus = ring.normalized_modulus();
        let mut vec = Vec::with_capacity(modulus.len());
        match x.into_repr() {
            Repr::Small(word) => vec.push(word),
            Repr::Large(mut words) => {
                if words.len() < modulus.len() {
                    vec.extend(&*words);
                } else {
                    let mut allocation = MemoryAllocation::new(div::memory_requirement_exact(
                        words.len(),
                        modulus.len(),
                    ));
                    let mut memory = allocation.memory();
                    let _overflow = div::div_rem_in_place(
                        &mut words,
                        modulus,
                        *ring.fast_div_top(),
                        &mut memory,
                    );
                    vec.extend(&words[..modulus.len()]);
                }
            }
        }
        vec.extend(iter::repeat(0).take(modulus.len() - vec.len()));
        ModuloLarge::new(vec, ring)
    }
}

/// Implement `IntoModulo` for unsigned primitives.
macro_rules! impl_into_modulo_for_unsigned {
    ($t:ty) => {
        impl IntoModulo for $t {
            fn into_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
                UBig::from(self).into_modulo(ring)
            }
        }
    };
}

/// Implement `IntoModulo` for signed primitives.
macro_rules! impl_into_modulo_for_signed {
    ($t:ty) => {
        impl IntoModulo for $t {
            fn into_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
                IBig::from(self).into_modulo(ring)
            }
        }
    };
}

impl_into_modulo_for_unsigned!(bool);
impl_into_modulo_for_unsigned!(u8);
impl_into_modulo_for_unsigned!(u16);
impl_into_modulo_for_unsigned!(u32);
impl_into_modulo_for_unsigned!(u64);
impl_into_modulo_for_unsigned!(u128);
impl_into_modulo_for_unsigned!(usize);
impl_into_modulo_for_signed!(i8);
impl_into_modulo_for_signed!(i16);
impl_into_modulo_for_signed!(i32);
impl_into_modulo_for_signed!(i64);
impl_into_modulo_for_signed!(i128);
impl_into_modulo_for_signed!(isize);
