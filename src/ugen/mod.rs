use std::ops::{Add, Mul};
use std::f64::consts::PI;
use crate::rand::Rng;

pub mod envelope;

pub trait ValueAt {
    fn value_at(&self, clock: f64) -> f64;
}

pub trait Duration {
    fn duration(&self) -> f64;
}

pub struct UGen<T> where T: ValueAt {
    parameters: T,
}

impl<T> ValueAt for UGen<T> where T: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        self.parameters.value_at(clock)
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

/*
TODO: implement waves
Self::Saw(frequency, phase) => (((clock + phase) * frequency) % 1.),
Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
*/

pub struct Sine {
    frequency: f64,
    phase: f64,
}

impl ValueAt for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase) * self.frequency * 2.0 * PI).sin()
    }
}

pub struct WhiteNoise { }

impl ValueAt for WhiteNoise {
    fn value_at(&self, _clock: f64) -> f64 {
        rand::thread_rng().gen_range(-1., 1.)
    }
}

pub struct Line {
    start: f64,
    end: f64,
    duration: f64
}

impl ValueAt for Line {
    fn value_at(&self, clock: f64) -> f64 {
        (self.start + (clock * (self.end - self.start) / self.duration))
    }
}


impl UGen<Sine> {
    pub(crate) fn sine(frequency: f64, phase: f64) -> UGen<Sine> {
        UGen {
            parameters: Sine { frequency, phase },
        }
    }

    pub(crate) fn white_noise() -> UGen<WhiteNoise> {
        UGen {
            parameters: WhiteNoise { },
        }
    }

    pub(crate) fn line(start: f64, end: f64, duration: f64) -> UGen<Line> {
        UGen {
            parameters: Line { start, end, duration },
        }
    }

}

impl From<f64> for UGen<Constant<f64>> {
    fn from(value: f64) -> Self {
        UGen {
            parameters: Constant { value },
        }
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