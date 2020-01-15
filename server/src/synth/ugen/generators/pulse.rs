use crate::synth::ugen::{ValueAt, UGen};
use crate::synth::ugen::params::Frequency;

pub struct Pulse {
    frequency: Box<dyn ValueAt>,
    phase: Box<dyn ValueAt>,
    width: Box<dyn ValueAt>,
}

impl Pulse {
    pub fn default() -> Self {
        Pulse {
            frequency: Box::new(UGen::from(440.)),
            phase: Box::new(UGen::from(0.)),
            width: Box::new(UGen::from(0.5))
        }
    }
}

impl ValueAt for Pulse {
    fn value_at(&self, clock: f64) -> f64 {
        if ((clock + self.phase.value_at(clock)) * self.frequency.value_at(clock)) % 1. < self.width.value_at(clock) {1.} else {-1.}
    }
}

impl<T> Frequency<T> for Pulse where T: 'static + ValueAt {
    fn frequency(self, value: UGen<T>) -> Self {
        Pulse {
            frequency: Box::new(value),
            ..self
        }
    }
}

impl UGen<Pulse> {
    pub fn width<T>(self, width: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            signal: Pulse {
                width: Box::new(width),
                ..self.signal
            },
            ..self
        }
    }
}