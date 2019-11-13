use std::ops::{Add, Mul};
use std::f64::consts::PI;
use crate::rand::Rng;
use std::collections::HashMap;

pub mod envelope;

type ValueAt = dyn Fn(f64) -> f64;

#[derive(Hash, PartialEq, Eq)]
enum UGenParameters {
    Function,
    Phase,
}

// TODO: add parameters
pub struct UGen {
    parameters: HashMap<UGenParameters, f64>,
    duration: f64,
    value_at: Box<ValueAt>,
}

/*
TODO: implement waves
Self::Saw(frequency, phase) => (((clock + phase) * frequency) % 1.),
Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
*/

impl UGen {
    pub(crate) fn sine(frequency: f64, phase: f64) -> Self {
        UGen {
            parameters: HashMap::new(),
            duration: 0.,
            value_at: Box::new(move |clock: f64| ((clock + phase) * frequency * 2.0 * PI).sin()),
        }
    }

    pub(crate) fn ar(attack: f64, release: f64, curve: f64) -> Self {
        envelope::ar(attack, release, curve)
    }

    pub(crate) fn white_noise() -> Self {
        UGen {
            parameters: HashMap::new(),
            duration: 0.,
            value_at: Box::new(move |_clock: f64| rand::thread_rng().gen_range(-1., 1.))
        }
    }

    pub(crate) fn line(start: f64, end: f64, duration: f64) -> Self {
        UGen {
            parameters: HashMap::new(),
            duration,
            value_at: Box::new(move |clock: f64| (start + (clock * (end - start)/duration)))
        }
    }

    pub(crate) fn value_at(&self, clock: f64) -> f64 {
        (self.value_at)(clock)
    }

    pub(crate) fn duration(&self) -> f64 {
        self.duration
    }
}

impl From<f64> for UGen {
    fn from(value: f64) -> Self {
        UGen {
            parameters: HashMap::new(),
            duration: 0.,
            value_at: Box::new(move |_clock: f64| value)
        }
    }
}

impl Add for UGen {
    type Output = UGen;

    fn add(self, rhs: Self) -> Self::Output {
        UGen {
            parameters: HashMap::new(),
            duration: self.duration.max(rhs.duration),
            value_at: Box::new(move |clock| self.value_at(clock) + rhs.value_at(clock))
        }
    }
}

impl Mul for UGen {
    type Output = UGen;

    fn mul(self, rhs: Self) -> Self::Output {
        UGen {
            parameters: HashMap::new(),
            duration: self.duration.max(rhs.duration),
            value_at: Box::new(move |clock| self.value_at(clock) * rhs.value_at(clock))
        }
    }
}