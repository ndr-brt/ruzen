use crate::clock::Hz;
use std::f64::consts::PI;

pub struct Amplitude {
    pub min: f64,
    pub max: f64,
}

pub enum Wave {
    None,
    Sine(Hz, f64),
    Saw(Hz, f64)
}

pub trait Oscillator {
    fn signal(&self, time: f64, frequency_scale: Hz, phase: f64) -> f64;
    fn frequency(&self) -> Hz;
}

impl dyn Oscillator {
    pub fn new(wave: Wave) -> Box<dyn Oscillator> {
        match wave {
            Wave::Sine(frequency, phase) => Box::new(Sine { frequency, phase }),
            Wave::Saw(frequency, phase) => Box::new(Saw { frequency, phase }),
            Wave::None => Box::new(None)
        }
    }
}

pub struct None;
impl Oscillator for None {
    fn signal(&self, time: f64, frequency: Hz, phase: f64) -> f64 {
        0.
    }

    fn frequency(&self) -> f64 {
        0.
    }
}

pub struct Sine {
    frequency: Hz,
    phase: f64,
}
impl Oscillator for Sine {
    fn signal(&self, time: f64, frequency: Hz, phase: f64) -> f64 {
        ((time + self.phase) * frequency * 2.0 * PI).sin()
    }

    fn frequency(&self) -> f64 {
        self.frequency
    }
}

pub struct Saw {
    frequency: Hz,
    phase: f64,
}
impl Oscillator for Saw {
    fn signal(&self, time: f64, frequency: Hz, phase: f64) -> f64 {
        (((time + phase) * frequency) % 1.)
    }

    fn frequency(&self) -> f64 {
        self.frequency
    }
}
