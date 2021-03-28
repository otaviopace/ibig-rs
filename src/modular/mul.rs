use crate::{
    arch::word::Word,
    div,
    memory::{self, Memory, MemoryAllocation},
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
        modulo_ring::ModuloRingLarge,
    },
    mul,
    primitive::extend_word,
    shift,
    sign::Sign::Positive,
};
use alloc::alloc::Layout;
use core::ops::{Mul, MulAssign};

impl<'a> Mul<Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(self, rhs: Modulo<'a>) -> Modulo<'a> {
        self.mul(&rhs)
    }
}

impl<'a> Mul<&Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(mut self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.mul_assign(rhs);
        self
    }
}

impl<'a> Mul<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(self, rhs: Modulo<'a>) -> Modulo<'a> {
        rhs.mul(self)
    }
}

impl<'a> Mul<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.clone().mul(rhs)
    }
}

impl<'a> MulAssign<Modulo<'a>> for Modulo<'a> {
    fn mul_assign(&mut self, rhs: Modulo<'a>) {
        self.mul_assign(&rhs)
    }
}

impl<'a> MulAssign<&Modulo<'a>> for Modulo<'a> {
    fn mul_assign(&mut self, rhs: &Modulo<'a>) {
        match (self.repr_mut(), rhs.repr()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.check_same_ring(rhs_small);
                self_small.mul_in_place(rhs_small);
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.check_same_ring(rhs_large);
                let memory_requirement = self_large.ring().mul_memory_requirement();
                let mut allocation = MemoryAllocation::new(memory_requirement);
                let mut memory = allocation.memory();
                self_large.mul_in_place(rhs_large, &mut memory);
            }
            _ => Modulo::panic_different_rings(),
        }
    }
}

impl<'a> ModuloSmall<'a> {
    /// self *= rhs
    pub(crate) fn mul_in_place(&mut self, rhs: &ModuloSmall<'a>) {
        self.mul_by_normalized_value_in_place(rhs.normalized_value());
    }

    /// self *= self
    pub(crate) fn square_in_place(&mut self) {
        self.mul_by_normalized_value_in_place(self.normalized_value());
    }

    fn mul_by_normalized_value_in_place(&mut self, normalized_value: Word) {
        let ring = self.ring();
        let self_val = self.normalized_value() >> ring.shift();
        let product = extend_word(self_val) * extend_word(normalized_value);
        let (_, product) = ring.fast_div().div_rem(product);
        self.set_normalized_value(product);
    }
}

impl ModuloRingLarge {
    pub(crate) fn mul_memory_requirement(&self) -> Layout {
        let n = self.normalized_modulus().len();
        memory::add_layout(
            memory::array_layout::<Word>(2 * n),
            memory::max_layout(
                mul::memory_requirement_exact(n),
                div::memory_requirement_exact(2 * n, n),
            ),
        )
    }

    /// Returns a * b allocated in memory.
    pub(crate) fn mul_normalized_values<'a>(
        &self,
        a: &[Word],
        b: &[Word],
        memory: &'a mut Memory,
    ) -> &'a [Word] {
        let modulus = self.normalized_modulus();
        let n = modulus.len();
        debug_assert!(a.len() == n && b.len() == n);

        let (product, mut memory) = memory.allocate_slice_fill::<Word>(2 * n, 0);
        let overflow = mul::add_signed_mul_same_len(product, Positive, a, b, &mut memory);
        assert_eq!(overflow, 0);
        shift::shr_in_place(product, self.shift());

        let _overflow = div::div_rem_in_place(product, modulus, *self.fast_div_top(), &mut memory);
        &product[..n]
    }
}

impl<'a> ModuloLarge<'a> {
    /// self *= rhs
    pub(crate) fn mul_in_place(&mut self, rhs: &ModuloLarge<'a>, memory: &mut Memory) {
        self.mul_normalized_value_in_place(rhs.normalized_value(), memory);
    }

    /// self *= self
    pub(crate) fn square_in_place(&mut self, memory: &mut Memory) {
        self.modify_normalized_value(|words, ring| {
            words.copy_from_slice(ring.mul_normalized_values(words, words, memory));
        });
    }

    /// self *= rhs
    pub(crate) fn mul_normalized_value_in_place(
        &mut self,
        normalized_value: &[Word],
        memory: &mut Memory,
    ) {
        self.modify_normalized_value(|words, ring| {
            words.copy_from_slice(ring.mul_normalized_values(words, normalized_value, memory));
        });
    }
}
