use crate::clock::{Clock};
use crate::ugen::{UGen, ValueAt, SignalRange};
use crate::ugen::envelope::Envelope;
use crate::ugen::generator::{Generator};
use std::ops::Mul;
use crate::synth::Parameters;

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

pub(crate) fn kick(sample_rate: f64, params: Parameters) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0001, 0.09, -4.)),
        signal: {
            Box::new(Generator::sine()
                .frequency(Envelope::ar(0.0001, 1.5, -200.).range(45., 845.))
                .phase(UGen::from(1.))
                * Envelope::line(1., 0., 1.)
            )
        },
    })
}

pub(crate) fn snare(sample_rate: f64, params: Parameters) -> Box<dyn Instrument> {
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

pub(crate) fn strange(sample_rate: f64, params: Parameters) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.1, 1.2, 4.)),
        signal: Box::new(
        Generator::saw().frequency(UGen::from(120.)) * UGen::from(0.5) +
            Generator::sine().frequency(UGen::from(100.)) * UGen::from(0.5)
        )
    })
}

pub(crate) fn catta(sample_rate: f64, params: Parameters) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(1., 0.2, 0.)),
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
    let freq = match params.get("freq") {
        Some(val) => UGen::from(val.to_owned().double().unwrap()),
        None => UGen::from(440.)
    };

    let phase = match params.get("phase") {
        Some(val) => UGen::from(val.to_owned().double().unwrap()),
        None => UGen::from(0.)
    };

    Box::new(ContinuousInstrument {
        clock: Clock::new(sample_rate),
        signal: Box::new(
            Generator::sine()
                .frequency(freq)
                .phase(phase)
        )
    })
}
