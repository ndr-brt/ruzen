use crate::envelope::Envelope;
use crate::clock::{Clock};
use crate::signal::Signal;

#[derive(PartialEq, Debug)]
pub enum Instruments {
    Kick, Snare
}

pub trait Instrument {
    fn signal(&mut self) -> f64;
    fn is_finished(&self) -> bool;
}

pub struct Kick {
    envelope: Envelope,
    clock: Clock
}
impl Instrument for Kick {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let modulation = Envelope::AR(0.0001, 1.5, -200.).value_at(self.clock.get());
        let signal = Signal::Sine((modulation * 800. + 45.), 1.).value_at(self.clock.get()) *
            Signal::Line(1.0, 0., 1.).value_at(self.clock.get());
        signal * self.envelope.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn kick(sample_rate: f64) -> Kick {
    Kick {
        clock: Clock::new(sample_rate),
        envelope: Envelope::AR(0.0001, 0.09, -4.)
    }
}

pub struct Snare {
    envelope: Envelope,
    clock: Clock
}
impl Instrument for Snare {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let snare =
            (Signal::Sine(30., 0.).value_at(self.clock.get()) * Envelope::AR(0.0005, 0.055, -4.).value_at(self.clock.get()) * 0.25)
            + (Signal::Sine(285., 0.).value_at(self.clock.get()) * Envelope::AR(0.0005, 0.075, -4.).value_at(self.clock.get()) * 0.25)
            + Signal::WhiteNoise().value_at(self.clock.get()) * 0.8;

        snare * self.envelope.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn snare(sample_rate: f64) -> Snare {
    Snare {
        clock: Clock::new(sample_rate),
        envelope: Envelope::AR(0.0005, 0.2, -4.)
    }
}