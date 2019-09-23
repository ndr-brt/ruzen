use crate::envelope::Envelope;
use crate::clock::{Clock};
use crate::signal::Signal;

#[derive(PartialEq, Debug)]
pub enum Instruments {
    Kick, Snare
}

pub trait Play {
    fn signal(&mut self) -> f64;
    fn is_finished(&self) -> bool;
}

pub struct Kick {
    envelope: Envelope,
    clock: Clock
}
impl Play for Kick {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = Signal::Sine(65., 0.).value_at(self.clock.get()) *
            Signal::Line(1.0, 0., 1.).value_at(self.clock.get());
        signal * self.envelope.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn kick(sample_rate: f64) -> Kick {
    println!("NEW KICK!");
    Kick {
        clock: Clock::new(sample_rate),
        envelope: Envelope::AR(0.01, 1., -4.)
    }
}

pub struct Snare {
    envelope: Envelope,
    clock: Clock
}
impl Play for Snare {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = Signal::Pulse(165., 0.).value_at(self.clock.get()) *
            Signal::Line(1.0, 0., 1.).value_at(self.clock.get());
        signal * self.envelope.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}
pub(crate) fn snare(sample_rate: f64) -> Snare {
    Snare {
        clock: Clock::new(sample_rate),
        envelope: Envelope::AR(0.05, 1., -4.)
    }
}