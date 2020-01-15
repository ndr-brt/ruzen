use crate::synth::ugen::{ValueAt, UGen};
use crate::synth::ugen::params::FrequencyParam;
use std::f64::consts::PI;

pub struct Sine {
    frequency: Box<dyn ValueAt>,
    phase: Box<dyn ValueAt>,
}

impl Sine {
    pub fn default() -> Self {
        Sine {
            frequency: Box::new(UGen::from(440.)),
            phase: Box::new(UGen::from(0.))
        }
    }
}

impl ValueAt for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase.value_at(clock)) * self.frequency.value_at(clock) * 2.0 * PI).sin()
    }
}

impl<T> FrequencyParam<T> for Sine where T: 'static + ValueAt {
    fn frequency(self, value: UGen<T>) -> Self {
        Sine {
            frequency: Box::new(value),
            ..self
        }
    }
}

impl UGen<Sine> {

    pub fn phase<T>(self, phase: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            signal: Sine {
                phase: Box::new(phase),
                ..self.signal
            },
            ..self
        }
    }
}