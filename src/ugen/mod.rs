use std::ops::{Add, Mul};

pub mod generator;
pub mod envelope;

pub trait ValueAt {
    fn value_at(&self, clock: f64) -> f64;
}

pub struct UGen<T> where T: ValueAt {
    parameters: T,
}

impl<T> ValueAt for UGen<T> where T: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        self.parameters.value_at(clock)
    }
}

pub struct Summed<T, O> where T: ValueAt, O: ValueAt {
    first: UGen<T>,
    second: UGen<O>,
}

impl<T, O> ValueAt for Summed<T, O> where T: ValueAt, O: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        self.first.value_at(clock) + self.second.value_at(clock)
    }
}

pub struct Multiplied<T, O> where T: ValueAt, O: ValueAt {
    first: UGen<T>,
    second: UGen<O>,
}

impl<T, O> ValueAt for Multiplied<T, O> where T: ValueAt, O: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        self.first.value_at(clock) * self.second.value_at(clock)
    }
}

impl<T: 'static, O: 'static> Add<UGen<O>> for UGen<T> where T: ValueAt, O: ValueAt {
    type Output = UGen<Summed<T, O>>;

    fn add(self, other: UGen<O>) -> Self::Output {
        UGen {
            parameters: Summed { first: self, second: other },
        }
    }
}

impl<T: 'static, O: 'static> Mul<UGen<O>> for UGen<T> where T: ValueAt, O: ValueAt {
    type Output = UGen<Multiplied<T, O>>;

    fn mul(self, other: UGen<O>) -> Self::Output {
        UGen {
            parameters: Multiplied { first: self, second: other },
        }
    }
}

pub struct Constant<T> {
    value: T,
}

impl ValueAt for Constant<f64> {
    fn value_at(&self, _clock: f64) -> f64 {
        self.value
    }
}

impl From<f64> for UGen<Constant<f64>> {
    fn from(value: f64) -> Self {
        UGen {
            parameters: Constant { value },
        }
    }
}
