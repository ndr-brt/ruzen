use std::ops::{Add, Mul};
use std::f64::consts::PI;
use crate::rand::Rng;
use std::collections::HashMap;

pub mod envelope;

pub trait ValueAt {
    fn value_at(&self, clock: f64) -> f64;
}

pub trait Duration {
    fn duration(&self) -> f64;
}

// TODO: add parameters
pub struct UGen<T> {
    duraaaa: f64,
    parameters: T,
    value: Box<dyn Fn(f64) -> f64>,
}

impl<T> ValueAt for UGen<T> {
    fn value_at(&self, clock: f64) -> f64 {
        (self.value)(clock)
    }
}

pub struct Sine {
    frequency: f64,
    phase: f64,
}

impl ValueAt for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase) * self.frequency * 2.0 * PI).sin()
    }
}

pub struct Combined {

}

/*
TODO: implement waves
Self::Saw(frequency, phase) => (((clock + phase) * frequency) % 1.),
Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
*/

impl UGen<Sine> {
    pub(crate) fn sine(frequency: f64, phase: f64) -> UGen<Sine> {
        UGen {
            parameters: Sine { frequency, phase },
            duraaaa: 0.,
            value: Box::new(move |clock: f64| ((clock + phase) * frequency * 2.0 * PI).sin()),
        }
    }

    pub(crate) fn white_noise() -> Self {
        UGen {
            parameters: Sine { frequency: 0., phase: 0. },
            duraaaa: 0.,
            value: Box::new(move |_clock: f64| rand::thread_rng().gen_range(-1., 1.))
        }
    }

    pub(crate) fn line(start: f64, end: f64, duration: f64) -> Self {
        UGen {
            parameters: Sine { frequency: 0., phase: 0. },
            duraaaa: duration,
            value: Box::new(move |clock: f64| (start + (clock * (end - start)/duration)))
        }
    }

    pub(crate) fn duration(&self) -> f64 {
        self.duraaaa
    }
}

impl From<f64> for UGen<Combined> {
    fn from(value: f64) -> Self {
        UGen {
            parameters: Combined {},
            duraaaa: 0.,
            value: Box::new(move |_clock: f64| value)
        }
    }
}

impl<T: 'static, O: 'static> Add<UGen<O>> for UGen<T> {
    type Output = UGen<Combined>;

    fn add(self, other: UGen<O>) -> Self::Output {
        UGen {
            parameters: Combined { },
            duraaaa: self.duraaaa.max(other.duraaaa),
            value: Box::new(move |clock| self.value_at(clock) + other.value_at(clock))
        }
    }
}

impl<T: 'static, O: 'static> Mul<UGen<O>> for UGen<T> {
    type Output = UGen<Combined>;

    fn mul(self, other: UGen<O>) -> Self::Output {
        UGen {
            parameters: Combined {  },
            duraaaa: self.duraaaa.max(other.duraaaa),
            value: Box::new(move |clock| self.value_at(clock) * other.value_at(clock))
        }
    }
}