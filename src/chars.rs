use core::marker::PhantomData;

/// Type level char list, or, well, "string"
pub trait Chars {
    const LEN: usize;
}

/// End of the type level string, not named Nil
struct EndOfString;
impl Chars for EndOfString {
    const LEN: usize = 0;
}

/// Some element of the type level list
struct Char<const C: char, Next: Chars>(PhantomData<Next>);
impl<const C: char, Next: Chars> Chars for Char<C, Next> {
    const LEN: usize = 1 + Next::LEN;
}
