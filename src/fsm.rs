pub const UNIVERSAL_START_STATE: usize = 0;
pub const UNIVERSAL_FAILED_STATE: usize = 1;

/// Type level Finite state machine
pub trait Fsm {
    const STATE_COUNT: usize;
    const START_STATE: usize = UNIVERSAL_START_STATE;
    const FAILED_STATE: usize = UNIVERSAL_FAILED_STATE;
    const TRANSITIONS: [fn(char) -> usize; Self::STATE_COUNT]
    fn transition(state: usize, transition: char) -> usize {
        Self::TRANSITIONS[state](transition)
    }
}
