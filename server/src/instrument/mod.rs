pub mod parameters;

use crate::clock::{Clock};
use std::ops::Mul;
use self::parameters::{Parameters, GetParameter};
use crate::synth::ugen::{UGen, ValueAt, SignalRange};
use crate::synth::ugen::envelope::Envelope;
use crate::synth::ugen::generator::Generator;
use crate::synth::ugen::params::*;

pub trait Instrument {
    fn signal(&mut self) -> f64;
    fn is_finished(&self) -> bool;
}

pub struct EnvelopedInstrument {
    envelope: Box<dyn Envelope>,
    clock: Clock,
    signal: Box<dyn ValueAt>,
}

impl Instrument for EnvelopedInstrument {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        self.signal.value_at(self.clock.get()) * self.envelope.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}

pub struct ContinuousInstrument {
    clock: Clock,
    signal: Box<dyn ValueAt>,
}

impl Instrument for ContinuousInstrument {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        self.signal.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        false
    }
}

pub(crate) fn kick(sample_rate: f64, _params: Parameters) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0001, 0.09, -4.)),
        signal: {
            Box::new(Generator::sine()
                .frequency(Envelope::ar(0.0001, 1.5, -200.).range(45., 845.))
                .phase(UGen::from(1.0))
                * Envelope::line(1., 0., 1.)
            )
        },
    })
}

pub(crate) fn snare(sample_rate: f64, _params: Parameters) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0005, 0.2, -4.)),
        signal: Box::new(
            (Generator::sine().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.055, -4.).range(0., 0.25))
            + (Generator::sine().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.075, -4.).range(0., 0.25))
            + (Generator::white_noise().mul(UGen::from(0.8)))
        )
    })
}

pub(crate) fn strange(sample_rate: f64, _params: Parameters) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.1, 1.2, 4.)),
        signal: Box::new(
        Generator::saw().frequency(UGen::from(120.)) * UGen::from(0.5) +
            Generator::sine().frequency(UGen::from(100.)) * UGen::from(0.5)
        )
    })
}

pub(crate) fn catta(sample_rate: f64, _params: Parameters) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.5, 0.2, 0.)),
        signal: {
            let first_width_modulation = Generator::sine().frequency(UGen::from(5.)).range(0.1, 0.9);
            let second_width_modulation = Generator::sine().frequency(UGen::from(1.4)).range(0.1, 0.9);

            Box::new(
                Generator::pulse().frequency(UGen::from(234.)).width(first_width_modulation) * UGen::from(0.5) +
                Generator::pulse().frequency(UGen::from(215.)).width(second_width_modulation) * UGen::from(0.5)
            )
        }
    })
}

pub(crate) fn sine(sample_rate: f64, params: Parameters) -> Box<dyn Instrument> {
    Box::new(ContinuousInstrument {
        clock: Clock::new(sample_rate),
        signal: Box::new(
            Generator::sine()
                .frequency(params.get("freq", UGen::from(440.)))
                .phase(params.get("phase", UGen::from(0.)))
        )
    })
}

pub(crate) fn saw(sample_rate: f64, params: Parameters) -> Box<dyn Instrument> {
    Box::new(ContinuousInstrument {
        clock: Clock::new(sample_rate),
        signal: Box::new(
            Generator::saw()
                .frequency(params.get("freq", UGen::from(440.)))
                .phase(params.get("phase", UGen::from(0.)))
        )
    })
}
