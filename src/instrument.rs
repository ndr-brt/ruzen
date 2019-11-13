use crate::clock::{Clock};
use crate::ugen::{UGen, ValueAt};
use crate::ugen::envelope::Envelope;
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
}
impl Instrument for Kick {
    fn signal(&mut self) -> f64 {
        self.clock.tick();

        let modulation = Envelope::ar(0.0001, 1.5, -200.) * UGen::from(800.) + UGen::from(45.);
        // TODO: make sine accept UGen, for modulation (need a function to scale value maybe)
        let signal = Sine::new(modulation.value_at(self.clock.get()), 1.)
            * Envelope::line(1., 0., 1.);

        signal.value_at(self.clock.get()) * self.envelope.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn kick(sample_rate: f64) -> Kick {
    Kick {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0001, 0.09, -4.))
    }
}

pub struct Snare {
    envelope: Box<dyn Envelope>,
    clock: Clock
}
impl Instrument for Snare {
    fn signal(&mut self) -> f64 {
        self.clock.tick();

        // TODO: implement range, could help for constants
        let snare =
            (Sine::new(30., 0.) * Envelope::ar(0.0005, 0.055, -4.) * UGen::from(0.25))
            + (Sine::new(285., 0.) * Envelope::ar(0.0005, 0.075, -4.) * UGen::from(0.25))
            + WhiteNoise::new() * UGen::from(0.8);

        snare.value_at(self.clock.get()) * self.envelope.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn snare(sample_rate: f64) -> Snare {
    Snare {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0005, 0.2, -4.))
    }
}
