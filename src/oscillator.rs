use crate::clock::Hz;
use std::f64::consts::PI;

pub struct Amplitude {
    pub min: f64,
    pub max: f64,
}

pub enum Wave {
    Const(Hz),
    Sine(Hz, f64),
    Saw(Hz, f64)
}

pub trait Oscillator {
    fn signal(&self, time: f64, frequency_scale: Hz, phase: f64) -> f64;
}

impl dyn Oscillator {
    pub fn new(wave: Wave) -> Box<dyn Oscillator> {
        match wave {
            Wave::Sine(frequency, phase) => Box::new(Sine { frequency, phase }),
            Wave::Saw(frequency, phase) => Box::new(Saw { frequency, phase }),
            Wave::Const(frequency) => Box::new(Const { frequency })
        }
    }
}

pub struct Const {
    frequency: Hz
}
impl Oscillator for Const {
    fn signal(&self, time: f64, frequency_scale: Hz, phase: f64) -> f64 {
        self.frequency
    }
}

pub struct Sine {
    frequency: Hz,
    phase: f64,
}
impl Oscillator for Sine {
    fn signal(&self, time: f64, frequency_scale: Hz, phase: f64) -> f64 {
        let modulated_frequency = self.frequency + (self.frequency * frequency_scale);
        ((time + self.phase) * modulated_frequency * 2.0 * PI).sin()
    }
}

pub struct Saw {
    frequency: Hz,
    phase: f64,
}
impl Oscillator for Saw {
    fn signal(&self, time: f64, frequency_scale: Hz, phase: f64) -> f64 {
        let modulated_frequency = self.frequency + (self.frequency * frequency_scale);
        (((time + phase) * modulated_frequency) % 1.)
    }
}
