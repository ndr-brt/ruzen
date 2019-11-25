use crate::rand::Rng;
use std::f64::consts::PI;
use crate::ugen::{ValueAt, UGen, Range};

/*
TODO: implement waves
Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
*/
const GENERATOR_RANGE: Range = Range { low: -1., high: 1. };

pub trait Generator: ValueAt {}

impl dyn Generator {
    pub fn sine() -> UGen<Sine> {
        UGen {
            parameters: Sine {
                frequency: Box::new(UGen::from(440.)),
                phase: Box::new(UGen::from(0.))
            },
            range: GENERATOR_RANGE,
        }
    }

    pub fn saw() -> UGen<Saw> {
        UGen {
            parameters: Saw {
                frequency: Box::new(UGen::from(440.)),
                phase: Box::new(UGen::from(0.))
            },
            range: GENERATOR_RANGE,
        }
    }

    pub fn white_noise() -> UGen<WhiteNoise> {
        UGen {
            parameters: WhiteNoise { },
            range: GENERATOR_RANGE,
        }
    }
}

pub struct Sine {
    frequency: Box<dyn ValueAt>,
    phase: Box<dyn ValueAt>,
}

impl ValueAt for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase.value_at(clock)) * self.frequency.value_at(clock) * 2.0 * PI).sin()
    }
}

impl UGen<Sine> {

    pub fn frequency<T>(self, frequency: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            parameters: Sine {
                frequency: Box::new(frequency),
                ..self.parameters
            },
            ..self
        }
    }

    pub fn phase<T>(self, phase: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            parameters: Sine {
                phase: Box::new(phase),
                ..self.parameters
            },
            ..self
        }
    }
}

pub struct Saw {
    frequency: Box<dyn ValueAt>,
    phase: Box<dyn ValueAt>,
}

impl ValueAt for Saw {
    fn value_at(&self, clock: f64) -> f64 {
        ((((clock + self.phase.value_at(clock)) * self.frequency.value_at(clock)) % 1.) - 0.5) * 2.
    }
}

impl UGen<Saw> {
    pub fn frequency<T>(self, frequency: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            parameters: Saw {
                frequency: Box::new(frequency),
                ..self.parameters
            },
            ..self
        }
    }
}

pub struct WhiteNoise { }

impl ValueAt for WhiteNoise {

    fn value_at(&self, _clock: f64) -> f64 {
        rand::thread_rng().gen_range(-1., 1.)
    }
}

#[cfg(test)]
mod tests {
    use crate::ugen::{ValueAt, UGen};
    use crate::ugen::generator::{Generator};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn sine() {
        let sine = Generator::sine().frequency(UGen::from(1.));

        assert_approx_eq!(sine.value_at(0.), 0.);
        assert_approx_eq!(sine.value_at(0.25), 1.);
        assert_approx_eq!(sine.value_at(0.5), 0.);
        assert_approx_eq!(sine.value_at(0.75), -1.);
        assert_approx_eq!(sine.value_at(1.), 0.);
    }

    #[test]
    fn saw() {
        let saw = Generator::saw().frequency(UGen::from(1.));

        assert_approx_eq!(saw.value_at(0.), -1.);
        assert_approx_eq!(saw.value_at(0.25), -0.5);
        assert_approx_eq!(saw.value_at(0.5), 0.);
        assert_approx_eq!(saw.value_at(0.75), 0.5);
        assert_approx_eq!(saw.value_at(1.), -1.);
    }

}