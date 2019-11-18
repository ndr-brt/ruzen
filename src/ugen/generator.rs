use crate::rand::Rng;
use std::f64::consts::PI;
use crate::ugen::{ValueAt, UGen, Range};

/*
TODO: implement waves
Self::Saw(frequency, phase) => (((clock + phase) * frequency) % 1.),
Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
*/
const GENERATOR_RANGE: Range = Range { low: -1., high: 1. };

pub struct Sine {
    frequency: f64,
    phase: f64,
}

impl Sine {
    pub(crate) fn new(frequency: f64, phase: f64) -> UGen<Self> {
        UGen {
            parameters: Sine { frequency, phase },
            range: GENERATOR_RANGE,
        }
    }
}

impl ValueAt for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase) * self.frequency * 2.0 * PI).sin()
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