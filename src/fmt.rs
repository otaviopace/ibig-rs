//! Formatting numbers.

use crate::{
    buffer::Buffer,
    div,
    ibig::IBig,
    primitive::{Word, WORD_BITS, WORD_BITS_USIZE},
    radix::{self, Digit, DigitCase},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use alloc::{format, string::String, vec::Vec};
use ascii::{AsciiChar, AsciiStr};
use core::{
    cmp::max,
    fmt::{self, Alignment, Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex, Write},
};

impl Display for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 10,
            prefix: "",
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Debug for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Octal for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl LowerHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl UpperHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Upper),
        }
        .fmt(f)
    }
}

impl Display for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 10,
            prefix: "",
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Debug for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Octal for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl LowerHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl UpperHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Upper),
        }
        .fmt(f)
    }
}

impl UBig {
    /// Representation in a given radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(format!("{}", ubig!(83).in_radix(3)), "10002");
    /// assert_eq!(format!("{:+010}", ubig!(35).in_radix(36)), "+00000000z");
    /// ```
    pub fn in_radix(&self, radix: u32) -> InRadix {
        radix::check_radix_valid(radix);
        InRadix {
            sign: Positive,
            magnitude: self,
            radix,
            prefix: "",
            digit_case: None,
        }
    }

    /// Deprecated: use `in_radix` instead.
    #[deprecated(since = "0.1.2", note = "use `in_radix` instead")]
    pub fn to_str_radix(&self, radix: u32) -> String {
        format!("{}", self.in_radix(radix))
    }

    /// Deprecated: use `in_radix` instead.
    #[deprecated(since = "0.1.2", note = "use `in_radix` instead")]
    pub fn to_str_radix_uppercase(&self, radix: u32) -> String {
        format!("{:#}", self.in_radix(radix))
    }
}

impl IBig {
    /// Representation in a given radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(format!("{}", ibig!(-83).in_radix(3)), "-10002");
    /// assert_eq!(format!("{:010}", ibig!(-35).in_radix(36)), "-00000000z");
    /// ```
    pub fn in_radix(&self, radix: u32) -> InRadix {
        radix::check_radix_valid(radix);
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix,
            prefix: "",
            digit_case: None,
        }
    }

    /// Deprecated: use `in_radix` instead.
    #[deprecated(since = "0.1.2", note = "use in_radix instead")]
    pub fn to_str_radix(&self, radix: u32) -> String {
        format!("{}", self.in_radix(radix))
    }

    /// Deprecated: use `in_radix` instead.
    #[deprecated(since = "0.1.2", note = "use in_radix instead")]
    pub fn to_str_radix_uppercase(&self, radix: u32) -> String {
        format!("{:#}", self.in_radix(radix))
    }
}

/// Representation of a `UBig` or `IBig` in any radix between 2 and 36 inclusive.
///
/// This can be used to format a number in a non-standard radix.
///
/// The default format uses lower-case letters a-z for digits 10-35.
/// The "alternative" format (`{:#}`) uses upper-case letters.
///
/// # Examples
///
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(format!("{}", ubig!(83).in_radix(3)), "10002");
/// assert_eq!(format!("{:+010}", ubig!(35).in_radix(36)), "+00000000z");
/// // For bases 2, 8, 10, 16 we don't have to use `InRadix`:
/// assert_eq!(format!("{:x}", ubig!(3000)), "bb8");
/// assert_eq!(format!("{:x}", ibig!(-3000)), "-bb8");
/// ```
pub struct InRadix<'a> {
    sign: Sign,
    magnitude: &'a UBig,
    radix: Digit,
    prefix: &'static str,
    digit_case: Option<DigitCase>,
}

impl Display for InRadix<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let digit_case = self.digit_case.unwrap_or_else(|| {
            if f.alternate() {
                DigitCase::Upper
            } else {
                DigitCase::Lower
            }
        });

        if self.radix.is_power_of_two() {
            match self.magnitude.repr() {
                Small(word) => {
                    let mut prepared = PreparedWordInPow2::new(*word, self.radix);
                    self.format_prepared(f, digit_case, &mut prepared)
                }
                Large(buffer) => {
                    let mut prepared = PreparedLargeInPow2::new(buffer, self.radix);
                    self.format_prepared(f, digit_case, &mut prepared)
                }
            }
        } else {
            match self.magnitude.repr() {
                Small(word) => {
                    let mut prepared = PreparedWordInNonPow2::new(*word, self.radix, digit_case, 1);
                    self.format_prepared(f, digit_case, &mut prepared)
                }
                Large(buffer) => {
                    let mut prepared = PreparedLargeInNonPow2::new(buffer, self.radix, digit_case);
                    self.format_prepared(f, digit_case, &mut prepared)
                }
            }
        }
    }
}

impl InRadix<'_> {
    /// Format using a `PreparedForFormatting`.
    fn format_prepared(
        &self,
        f: &mut Formatter,
        digit_case: DigitCase,
        prepared: &mut dyn PreparedForFormatting,
    ) -> fmt::Result {
        let mut width = prepared.width();

        // Adding sign and prefix to width will not overflow, because Buffer::MAX_CAPACITY leaves
        // (WORD_BITS - 1) spare bits before we would hit overflow.
        let sign = if self.sign == Negative {
            "-"
        } else if f.sign_plus() {
            "+"
        } else {
            ""
        };
        // In bytes, but it's OK because everything is ASCII.
        width += sign.len() + self.prefix.len();

        match f.width() {
            None => {
                f.write_str(sign)?;
                f.write_str(self.prefix)?;
                prepared.write(f, digit_case)?;
            }
            Some(min_width) => {
                if width >= min_width {
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    prepared.write(f, digit_case)?;
                } else if f.sign_aware_zero_pad() {
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    for _ in 0..min_width - width {
                        f.write_str("0")?;
                    }
                    prepared.write(f, digit_case)?;
                } else {
                    let left = match f.align() {
                        Some(Alignment::Left) => 0,
                        Some(Alignment::Right) | None => min_width - width,
                        Some(Alignment::Center) => (min_width - width) / 2,
                    };
                    let fill = f.fill();
                    for _ in 0..left {
                        f.write_char(fill)?;
                    }
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    prepared.write(f, digit_case)?;
                    for _ in left..min_width - width {
                        f.write_char(fill)?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// Trait for state of a partially-formatted `UBig`.
///
/// The state must be such the width (number of digits) is already known.
trait PreparedForFormatting {
    /// Returns the number of characters that will be written.
    fn width(&self) -> usize;

    /// Write to a stream.
    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result;
}

/// A `Word` prepared for formatting in a power-of-2 radix.
struct PreparedWordInPow2 {
    word: Word,
    log_radix: u32,
    width: usize,
}

impl PreparedWordInPow2 {
    /// Prepare a `Word` for formatting in a power-of-2 radix.
    fn new(word: Word, radix: Digit) -> PreparedWordInPow2 {
        debug_assert!(radix >= 2 && radix.is_power_of_two());
        let log_radix = radix.trailing_zeros();
        debug_assert!(log_radix <= WORD_BITS);
        let width = max(
            (WORD_BITS - word.leading_zeros() + log_radix - 1) / log_radix,
            1,
        ) as usize;

        PreparedWordInPow2 {
            word,
            log_radix,
            width,
        }
    }
}

impl PreparedForFormatting for PreparedWordInPow2 {
    fn width(&self) -> usize {
        self.width
    }

    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result {
        let mask: Digit = (1 << self.log_radix) - 1;
        let mut digits = [AsciiChar::Null; WORD_BITS_USIZE];
        for idx in 0..self.width {
            let digit = (self.word >> (idx as u32 * self.log_radix)) as Digit & mask;
            digits[self.width - 1 - idx] = radix::digit_to_ascii(digit, digit_case);
        }
        let s: &AsciiStr = digits[..self.width].into();
        writer.write_str(s.as_str())?;
        Ok(())
    }
}

/// A large number prepared for formatting in a power-of-2 radix.
struct PreparedLargeInPow2<'a> {
    words: &'a [Word],
    log_radix: u32,
    width: usize,
}

impl PreparedLargeInPow2<'_> {
    /// Prepare a large number for formatting in a power-of-2 radix.
    fn new(words: &[Word], radix: Digit) -> PreparedLargeInPow2 {
        debug_assert!(radix::is_radix_valid(radix) && radix.is_power_of_two());
        let log_radix = radix.trailing_zeros();
        debug_assert!(log_radix <= WORD_BITS);
        // No overflow because words.len() * WORD_BITS + (log_radix-1) <= usize::MAX for
        // words.len() <= Buffer::MAX_CAPACITY.
        let width = max(
            (words.len() * WORD_BITS_USIZE - words.last().unwrap().leading_zeros() as usize
                + (log_radix - 1) as usize)
                / log_radix as usize,
            1,
        );
        PreparedLargeInPow2 {
            words,
            log_radix,
            width,
        }
    }
}

impl PreparedForFormatting for PreparedLargeInPow2<'_> {
    fn width(&self) -> usize {
        self.width
    }

    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result {
        let mask: Digit = (1 << self.log_radix) - 1;

        let mut it = self.words.iter().rev();
        let mut word = it.next().unwrap();
        let mut bits = (self.width * self.log_radix as usize
            - (self.words.len() - 1) * WORD_BITS_USIZE) as u32;

        const MAX_BUFFER_LEN: usize = 32;
        let mut buffer = [AsciiChar::default(); MAX_BUFFER_LEN];
        let mut buffer_len = 0;

        loop {
            let digit;
            if bits < self.log_radix {
                match it.next() {
                    Some(w) => {
                        let extra_bits = self.log_radix - bits;
                        bits = WORD_BITS - extra_bits;
                        digit = (word << extra_bits | w >> bits) as Digit & mask;
                        word = w;
                    }
                    None => break,
                }
            } else {
                bits -= self.log_radix;
                digit = (word >> bits) as Digit & mask;
            }
            buffer[buffer_len] = radix::digit_to_ascii(digit, digit_case);
            buffer_len += 1;
            if buffer_len == MAX_BUFFER_LEN {
                let s: &AsciiStr = (&buffer[..]).into();
                writer.write_str(s.as_str())?;
                buffer_len = 0;
            }
        }
        debug_assert!(bits == 0);
        let s: &AsciiStr = (&buffer[..buffer_len]).into();
        writer.write_str(s.as_str())?;
        Ok(())
    }
}

/// A `Word` prepared for formatting in a non-power-of-2 radix.
struct PreparedWordInNonPow2 {
    // digits[start_index..] actually used.
    digits: [AsciiChar; radix::MAX_WORD_DIGITS_NON_POW_2],
    start_index: usize,
}

impl PreparedWordInNonPow2 {
    /// Prepare a `Word` for formatting in a non-power-of-2 radix.
    fn new(
        mut word: Word,
        radix: Digit,
        digit_case: DigitCase,
        min_digits: usize,
    ) -> PreparedWordInNonPow2 {
        debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
        let radix_info = radix::radix_info(radix);

        let mut prepared = PreparedWordInNonPow2 {
            digits: [AsciiChar::default(); radix::MAX_WORD_DIGITS_NON_POW_2],
            start_index: radix::MAX_WORD_DIGITS_NON_POW_2,
        };

        let max_start = radix::MAX_WORD_DIGITS_NON_POW_2 - min_digits;
        while prepared.start_index > max_start || word != 0 {
            let (new_word, d) = radix_info.fast_div_radix.div_rem(word);
            word = new_word;
            let ch = radix::digit_to_ascii(d as Digit, digit_case);
            prepared.start_index -= 1;
            prepared.digits[prepared.start_index] = ch;
        }

        prepared
    }
}

impl PreparedForFormatting for PreparedWordInNonPow2 {
    fn width(&self) -> usize {
        radix::MAX_WORD_DIGITS_NON_POW_2 - self.start_index
    }

    fn write(&mut self, writer: &mut dyn Write, _digit_case: DigitCase) -> fmt::Result {
        let s: &AsciiStr = self.digits[self.start_index..].into();
        writer.write_str(s.as_str())?;
        Ok(())
    }
}

/// A large number prepared for formatting in a non-power-of-2 radix.
struct PreparedLargeInNonPow2 {
    top_group: PreparedWordInNonPow2,
    // Little endian in groups of max digits per word.
    // TODO: Change to static array when recursive implemented.
    low_groups: Vec<Word>,
    radix: Digit,
}

impl PreparedLargeInNonPow2 {
    /// Prepare a large number for formatting in a non-power-of-2 radix.
    fn new(words: &[Word], radix: Digit, digit_case: DigitCase) -> PreparedLargeInNonPow2 {
        debug_assert!(words.len() >= 2 && radix::is_radix_valid(radix) && !radix.is_power_of_two());
        let radix_info = radix::radix_info(radix);

        // There is at most 1 extra digit per word beyond digits_per_word.
        // Max total extra words: ceil(words.len() / digits_per_word).
        // One of them is top_group.
        let mut low_groups =
            Vec::with_capacity(words.len() + words.len() / radix_info.digits_per_word);
        let mut buffer = Buffer::allocate_no_extra(words.len());
        buffer.extend(words);
        while buffer.len() > 1 {
            let rem =
                div::fast_div_by_word_in_place(&mut buffer, radix_info.fast_div_range_per_word);
            low_groups.push(rem);
            buffer.pop_leading_zeros();
        }
        assert!(buffer.len() == 1);
        PreparedLargeInNonPow2 {
            top_group: PreparedWordInNonPow2::new(buffer[0], radix, digit_case, 1),
            low_groups,
            radix,
        }
    }
}

impl PreparedForFormatting for PreparedLargeInNonPow2 {
    fn width(&self) -> usize {
        let radix_info = radix::radix_info(self.radix);
        self.top_group.width() + self.low_groups.len() * radix_info.digits_per_word
    }

    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result {
        let radix_info = radix::radix_info(self.radix);

        self.top_group.write(writer, digit_case)?;

        for group_word in self.low_groups.iter().rev() {
            let mut prepared = PreparedWordInNonPow2::new(
                *group_word,
                self.radix,
                digit_case,
                radix_info.digits_per_word,
            );
            prepared.write(writer, digit_case)?;
        }
        Ok(())
    }
}
