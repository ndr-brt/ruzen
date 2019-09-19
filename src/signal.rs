use std::f64::consts::PI;

pub enum Signal {
  Sine(f64, f64),
  Saw(f64, f64),
}

impl Signal {
    fn value_at(&self, clock: f64) -> f64 {
        match self {
            Self::Sine(frequency, phase) => ((clock + phase) * frequency * 2.0 * PI).sin(),
            Self::Saw(frequency, phase) => (((clock + phase) * frequency) % 1.)
        }
    }
}