use crate::oscillator::{ Oscillator };
use crate::envelope::Envelope;
use crate::clock::{Clock};
use crate::signal::Signal;
use std::any::Any;

#[derive(PartialEq, Debug)]
pub enum Instruments {
    Kick, Snare
}

pub trait Play {
    fn signal(&mut self) -> f64;
}

pub struct Sine {
    frequency: f64,
    clock: Clock
}
impl Play for Sine {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = Signal::Sine(self.frequency, 0.).value_at(self.clock.get());
        let envelope = Envelope::AR(1., 1.).value_at(self.clock.get());
        signal * envelope
    }
}
pub(crate) fn sine(sample_rate: f64, frequency: f64) -> Sine {
    Sine {
        clock: Clock::new(sample_rate),
        frequency,
    }
}

pub struct Saw {
    frequency: f64,
    clock: Clock
}
impl Play for Saw {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = Signal::Saw(self.frequency, 0.).value_at(self.clock.get());
        let envelope = Envelope::AR(1., 1.).value_at(self.clock.get());
        signal * envelope
    }
}
pub(crate) fn saw(sample_rate: f64, frequency: f64) -> Saw {
    Saw {
        clock: Clock::new(sample_rate),
        frequency,
    }
}

pub struct Kick {
    clock: Clock
}
impl Play for Kick {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = Signal::Sine(65., 0.).value_at(self.clock.get()) *
            Signal::Line(1.0, 0., 1.).value_at(self.clock.get());
        let envelope = Envelope::AR(0.05, 1.).value_at(self.clock.get());
        signal * envelope
    }
}
pub(crate) fn kick(sample_rate: f64) -> Kick {
    Kick {
        clock: Clock::new(sample_rate)
    }
}

pub struct Snare {
    clock: Clock
}
impl Play for Snare {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = Signal::Pulse(165., 0.).value_at(self.clock.get()) *
            Signal::Line(1.0, 0., 1.).value_at(self.clock.get());
        let envelope = Envelope::AR(0.05, 1.).value_at(self.clock.get());
        signal * envelope
    }
}
pub(crate) fn snare(sample_rate: f64) -> Snare {
    Snare {
        clock: Clock::new(sample_rate)
    }
}