use crate::rand::Rng;
use std::f64::consts::PI;
use crate::synth::ugen::{ValueAt, UGen, Range};
use crate::synth::ugen::params::Frequency;
use crate::synth::ugen::generators::sine::Sine;
use crate::synth::ugen::generators::saw::Saw;
use crate::synth::ugen::generators::pulse::Pulse;
use crate::synth::ugen::generators::whitenoise::WhiteNoise;

const GENERATOR_RANGE: Range = Range { low: -1., high: 1. };

pub trait Generator: ValueAt {}

impl dyn Generator {
    pub fn sine() -> UGen<Sine> {
        UGen {
            signal: Sine::default(),
            range: GENERATOR_RANGE,
        }
    }

    pub fn saw() -> UGen<Saw> {
        UGen {
            signal: Saw::default(),
            range: GENERATOR_RANGE,
        }
    }

    pub fn pulse() -> UGen<Pulse> {
        UGen {
            signal: Pulse::default(),
            range: GENERATOR_RANGE,
        }
    }

    pub fn white_noise() -> UGen<WhiteNoise> {
        UGen {
            signal: WhiteNoise::default(),
            range: GENERATOR_RANGE,
        }
    }
}

impl<T,O> Frequency<T> for UGen<O> where T: 'static + ValueAt, O: Frequency<T> + ValueAt {
    fn frequency(self, value: UGen<T>) -> Self {
        UGen {
            signal: self.signal.frequency(value),
            ..self
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::ugen::{ValueAt, UGen};
    use crate::ugen::generator::{Generator};
    use assert_approx_eq::assert_approx_eq;
    use crate::synth::ugen::UGen;

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