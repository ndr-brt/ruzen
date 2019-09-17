use crate::clock::Hz;
use std::f64::consts::PI;

pub enum Wave { Sine, Saw }

pub trait Oscillator {
    fn signal(&self, time: f64, frequency: Hz, phase: f64) -> f64;
}

impl dyn Oscillator {
    pub fn new(wave: Wave) -> Box<dyn Oscillator> {
        match wave {
            Wave::Sine => Box::new(Sine),
            Wave::Saw => Box::new(Saw),
        }
    }
}

pub struct Sine;
impl Oscillator for Sine {
    fn signal(&self, time: f64, frequency: Hz, phase: f64) -> f64 {
        ((time + phase) * frequency * 2.0 * PI).sin()
    }
}

pub struct Saw;
impl Oscillator for Saw {
    fn signal(&self, time: f64, frequency: Hz, phase: f64) -> f64 {
        (((time + phase) * frequency) % 1.)
    }
}
