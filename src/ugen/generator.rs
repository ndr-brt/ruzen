use crate::rand::Rng;
use std::f64::consts::PI;
use crate::ugen::{ValueAt, UGen, Range};

/*
TODO: implement waves
Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
*/
const GENERATOR_RANGE: Range = Range { low: -1., high: 1. };

pub struct Sine {
    frequency: Box<dyn ValueAt>,
    phase: Box<dyn ValueAt>,
}

impl Sine {
    pub(crate) fn new() -> UGen<Self> {
        UGen {
            parameters: Sine {
                frequency: Box::new(UGen::from(440.)),
                phase: Box::new(UGen::from(0.))
            },
            range: GENERATOR_RANGE,
        }
    }
}

impl UGen<Sine> {

    pub fn frequency<T>(self, frequency: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            parameters: Sine {
                frequency: Box::new(frequency),
                phase: self.parameters.phase,
            },
            range: self.range,
        }
    }

    pub fn phase<T>(self, phase: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            parameters: Sine {
                frequency: self.parameters.frequency,
                phase: Box::new(phase),
            },
            range: self.range,
        }
    }
}

impl ValueAt for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase.value_at(clock)) * self.frequency.value_at(clock) * 2.0 * PI).sin()
    }
}

pub struct Saw {
    frequency: f64,
    phase: f64,
}

impl Saw {
    fn new(frequency: f64, phase: f64) -> Self {
        Saw { frequency, phase }
    }
}

impl ValueAt for Saw {
    fn value_at(&self, clock: f64) -> f64 {
        ((((clock + self.phase) * self.frequency) % 1.) - 0.5) * 2.
    }
}

pub struct WhiteNoise { }

impl WhiteNoise {
    pub(crate) fn new() -> UGen<Self> {
        UGen {
            parameters: WhiteNoise { },
            range: GENERATOR_RANGE,
        }
    }
}

impl ValueAt for WhiteNoise {

    fn value_at(&self, _clock: f64) -> f64 {
        rand::thread_rng().gen_range(-1., 1.)
    }
}


#[cfg(test)]
mod tests {
    use crate::ugen::{ValueAt, SignalRange};
    use crate::ugen::generator::{Sine, Saw};
    use std::f64::consts::PI;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn sine() {
        let sine = Sine::new(1., 0.);

        assert_approx_eq!(sine.value_at(0.), 0.);
        assert_approx_eq!(sine.value_at(0.25), 1.);
        assert_approx_eq!(sine.value_at(0.5), 0.);
        assert_approx_eq!(sine.value_at(0.75), -1.);
        assert_approx_eq!(sine.value_at(1.), 0.);
    }

    #[test]
    fn saw() {
        let saw = Saw::new(1., 0.);

        assert_approx_eq!(saw.value_at(0.), -1.);
        assert_approx_eq!(saw.value_at(0.25), -0.5);
        assert_approx_eq!(saw.value_at(0.5), 0.);
        assert_approx_eq!(saw.value_at(0.75), 0.5);
        assert_approx_eq!(saw.value_at(1.), -1.);
    }

}