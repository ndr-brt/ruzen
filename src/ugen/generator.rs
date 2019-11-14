use crate::rand::Rng;
use std::f64::consts::PI;
use crate::ugen::{ValueAt, UGen};

/*
TODO: implement waves
Self::Saw(frequency, phase) => (((clock + phase) * frequency) % 1.),
Self::Pulse(frequency, phase) => if ((clock + phase) * frequency) % 1. < 0.5 {1.} else {-1.},
*/

pub struct Sine {
    frequency: f64,
    phase: f64,
}

impl Sine {
    pub(crate) fn new(frequency: f64, phase: f64) -> UGen<Self> {
        UGen { parameters: Sine { frequency, phase } }
    }
}

impl ValueAt for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase) * self.frequency * 2.0 * PI).sin()
    }
}

pub struct WhiteNoise { }

impl WhiteNoise {
    pub(crate) fn new() -> UGen<Self> {
        UGen { parameters: WhiteNoise { } }
    }
}

impl ValueAt for WhiteNoise {

    fn value_at(&self, _clock: f64) -> f64 {
        rand::thread_rng().gen_range(-1., 1.)
    }
}


#[cfg(test)]
mod tests {
    use crate::ugen::{ValueAt, Range};
    use crate::ugen::generator::Sine;
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

}