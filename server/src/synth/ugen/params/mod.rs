use crate::synth::ugen::{ValueAt, UGen};

pub trait Frequency<T> where T: ValueAt {
    fn frequency(self, value: UGen<T>) -> Self;
}
