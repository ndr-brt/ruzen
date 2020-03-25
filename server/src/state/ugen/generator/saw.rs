use crate::state::ugen::{ValueAt, UGen};
use crate::state::ugen::params::{FrequencyParam, PhaseParam};

pub struct Saw {
    frequency: Box<dyn ValueAt>,
    phase: Box<dyn ValueAt>,
}

impl Saw {
    pub fn default() -> Self {
        Saw {
            frequency: Box::new(UGen::from(440.)),
            phase: Box::new(UGen::from(0.))
        }
    }
}

impl ValueAt for Saw {
    fn value_at(&self, clock: f64) -> f64 {
        ((((clock + self.phase.value_at(clock)) * self.frequency.value_at(clock)) % 1.) - 0.5) * 2.
    }
}

impl<T> FrequencyParam<T> for Saw where T: 'static + ValueAt {
    fn frequency(self, value: UGen<T>) -> Self {
        Saw {
            frequency: Box::new(value),
            ..self
        }
    }
}

impl<T> PhaseParam<T> for Saw where T: 'static + ValueAt {
    fn phase(self, value: UGen<T>) -> Self {
        Saw {
            phase: Box::new(value),
            ..self
        }
    }
}