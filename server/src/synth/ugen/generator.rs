use crate::rand::Rng;
use std::f64::consts::PI;
use crate::synth::ugen::{ValueAt, UGen, Range};

const GENERATOR_RANGE: Range = Range { low: -1., high: 1. };

pub trait Generator: ValueAt {}

impl dyn Generator {
    pub fn sine() -> UGen<Sine> {
        UGen {
            signal: Sine {
                frequency: Box::new(UGen::from(440.)),
                phase: Box::new(UGen::from(0.))
            },
            range: GENERATOR_RANGE,
        }
    }

    pub fn saw() -> UGen<Saw> {
        UGen {
            signal: Saw {
                frequency: Box::new(UGen::from(440.)),
                phase: Box::new(UGen::from(0.))
            },
            range: GENERATOR_RANGE,
        }
    }

    pub fn pulse() -> UGen<Pulse> {
        UGen {
            signal: Pulse {
                frequency: Box::new(UGen::from(440.)),
                phase: Box::new(UGen::from(0.)),
                width: Box::new(UGen::from(0.5)),
            },
            range: GENERATOR_RANGE,
        }
    }

    pub fn white_noise() -> UGen<WhiteNoise> {
        UGen {
            signal: WhiteNoise { },
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
            signal: Sine {
                frequency: Box::new(frequency),
                ..self.signal
            },
            ..self
        }
    }

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
            signal: Saw {
                frequency: Box::new(frequency),
                ..self.signal
            },
            ..self
        }
    }
}

pub struct Pulse {
    frequency: Box<dyn ValueAt>,
    phase: Box<dyn ValueAt>,
    width: Box<dyn ValueAt>,
}

impl ValueAt for Pulse {
    fn value_at(&self, clock: f64) -> f64 {
        if ((clock + self.phase.value_at(clock)) * self.frequency.value_at(clock)) % 1. < self.width.value_at(clock) {1.} else {-1.}
    }
}

impl UGen<Pulse> {
    // TODO: there's a way to remove this code to make this simpler?
    pub fn frequency<T>(self, frequency: UGen<T>) -> Self where T: 'static + ValueAt {
        UGen {
            signal: Pulse {
                frequency: Box::new(frequency),
                ..self.signal
            },
            ..self
        }
    }

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