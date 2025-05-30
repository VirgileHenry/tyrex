use crate::fsm::Fsm;

struct EndOfRegex;

impl Fsm for EndOfRegex {
    const STATE_COUNT: usize = 2;
    const TRANSITIONS: [fn(char) -> usize; Self::STATE_COUNT] = [
        |_| crate::fsm::UNIVERSAL_FAILED_STATE,
        |_| crate::fsm::UNIVERSAL_FAILED_STATE,
    ];
}

struct Char<const C: char, Next: Fsm>(core::marker::PhantomData<Next>);

impl<const C: char, Next: Fsm> Fsm for Char<C, Next> {
    const STATE_COUNT: usize = Next::STATE_COUNT + 1;
    const TRANSITIONS: [fn(char) -> usize; Self::STATE_COUNT] = [|_| 0; Self::STATE_COUNT];
}
