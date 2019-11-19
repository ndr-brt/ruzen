use crate::clock::{Clock};
use crate::ugen::{UGen, ValueAt, SignalRange};
use crate::ugen::envelope::Envelope;
use crate::plot::{Plot};
use crate::ugen::generator::{Sine, WhiteNoise};

#[derive(PartialEq, Debug)]
pub enum Instruments {
    Kick, Snare,
}

pub trait Instrument {
    fn signal(&mut self) -> f64;
    fn is_finished(&self) -> bool;
}

pub struct Kick {
    envelope: Box<dyn Envelope>,
    clock: Clock,
    signal: Box<dyn ValueAt>,
}
impl Instrument for Kick {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        self.signal.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn kick(sample_rate: f64) -> Kick {
    Kick {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0001, 0.09, -4.)),
        signal: {
            let signal = Sine::new()
                .frequency(Envelope::ar(0.0001, 1.5, -200.).range(45., 845.))//.plot()
                .phase(UGen::from(1.))
                    * Envelope::line(1., 0., 1.);

            Box::new(signal * Envelope::ar(0.0001, 0.09, -4.))
        },
    }
}

pub struct Snare {
    envelope: Box<dyn Envelope>,
    clock: Clock,
    signal: Box<dyn ValueAt>,
}
impl Instrument for Snare {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        self.signal.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn snare(sample_rate: f64) -> Snare {
    Snare {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0005, 0.2, -4.)),
        signal: {
            let snare =
                (Sine::new().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.055, -4.).range(0., 0.25))
                    + (Sine::new().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.075, -4.).range(0., 0.25))
                    + WhiteNoise::new() * UGen::from(0.8); // TODO: maybe here a "mul" function will be more expressive

            Box::new(snare * Envelope::ar(0.0005, 0.2, -4.))
        }
    }
}
