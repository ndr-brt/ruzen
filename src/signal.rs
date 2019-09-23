use std::f64::consts::PI;
use crate::rand::Rng;

pub enum Signal {
  Sine(f64, f64),
  Saw(f64, f64),
  Line(f64, f64, f64),
  Pulse(f64, f64),
  WhiteNoise(),
}

impl Signal {
    pub(crate) fn value_at(&self, clock: f64) -> f64 {
        match self {
            Self::Sine(frequency, phase) => ((clock + phase) * frequency * 2.0 * PI).sin(),
            Self::Saw(frequency, phase) => (((clock + phase) * frequency) % 1.),
            Self::Line(start, end, duration) => (start + (clock * (end - start)/duration)),
            Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
            Self::WhiteNoise() => rand::thread_rng().gen_range(-1., 1.),
        }
    }
}