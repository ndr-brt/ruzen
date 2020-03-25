use crate::state::ugen::{ValueAt, UGen};
use crate::state::ugen::params::{FrequencyParam, WidthParam, PhaseParam};

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

impl<T> FrequencyParam<T> for Pulse where T: 'static + ValueAt {
    fn frequency(self, value: UGen<T>) -> Self {
        Pulse {
            frequency: Box::new(value),
            ..self
        }
    }
}

impl<T> PhaseParam<T> for Pulse where T: 'static + ValueAt {
    fn phase(self, value: UGen<T>) -> Self {
        Pulse {
            phase: Box::new(value),
            ..self
        }
    }
}

impl<T> WidthParam<T> for Pulse where T: 'static + ValueAt {
    fn width(self, value: UGen<T>) -> Self {
        Pulse {
            width: Box::new(value),
            ..self
        }
    }
}