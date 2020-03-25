use crate::state::ugen::{ValueAt, UGen};

pub trait FrequencyParam<T> where T: ValueAt {
    fn frequency(self, value: UGen<T>) -> Self;
}

pub trait WidthParam<T> where T: ValueAt {
    fn width(self, value: UGen<T>) -> Self;
}

pub trait PhaseParam<T> where T: ValueAt {
    fn phase(self, value: UGen<T>) -> Self;
}
