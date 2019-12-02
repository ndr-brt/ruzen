use crate::clock::{Clock};
use crate::ugen::{UGen, ValueAt, SignalRange};
use crate::ugen::envelope::Envelope;
use crate::plot::{Plot};
use crate::ugen::generator::{Generator};

pub struct EnvelopedInstrument {
    envelope: Box<dyn Envelope>,
    clock: Clock,
    signal: Box<dyn ValueAt>,
}

impl Instrument for EnvelopedInstrument {
    fn signal(&mut self) -> f64 {
        self.clock.tick();
        self.signal.value_at(self.clock.get())
    }

    fn is_finished(&self) -> bool {
        self.clock.get() > self.envelope.duration()
    }
}

pub trait Instrument {
    fn signal(&mut self) -> f64;
    fn is_finished(&self) -> bool;
}

pub(crate) fn kick(sample_rate: f64) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0001, 0.09, -4.)),
        signal: {
            let signal = Generator::sine()
                .frequency(Envelope::ar(0.0001, 1.5, -200.).range(45., 845.))//.plot()
                .phase(UGen::from(1.))
                    * Envelope::line(1., 0., 1.);

            Box::new(signal * Envelope::ar(0.0001, 0.09, -4.))
        },
    })
}

pub(crate) fn snare(sample_rate: f64) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.0005, 0.2, -4.)),
        signal: {
            let snare =
                (Generator::sine().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.055, -4.).range(0., 0.25))
                    + (Generator::sine().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.075, -4.).range(0., 0.25))
                    + (Generator::white_noise() * UGen::from(0.8)); // TODO: maybe here a "mul" function will be more expressive

            Box::new(snare * Envelope::ar(0.0005, 0.2, -4.))
        }
    })
}

pub(crate) fn strange(sample_rate: f64) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(0.1, 1.2, 4.)),
        signal: {
            let signal = (
                Generator::saw().frequency(UGen::from(120.)) * UGen::from(0.5) +
                    Generator::sine().frequency(UGen::from(100.)) * UGen::from(0.5)
            );

            Box::new(signal * Envelope::ar(0.1, 1.2, 4.))
        }
    })
}

pub(crate) fn catta(sample_rate: f64) -> Box<dyn Instrument> {
    Box::new(EnvelopedInstrument {
        clock: Clock::new(sample_rate),
        envelope: Box::new(Envelope::ar(1., 0.2, 0.)),
        signal: {
            let first_width_modulation = Generator::sine().frequency(UGen::from(5.)).range(0.1, 0.9);
            let second_width_modulation = Generator::sine().frequency(UGen::from(1.4)).range(0.1, 0.9);
            let signal = (
                Generator::pulse().frequency(UGen::from(234.)).width(first_width_modulation) * UGen::from(0.5) +
                Generator::pulse().frequency(UGen::from(215.)).width(second_width_modulation) * UGen::from(0.5)
            );

            Box::new(signal * Envelope::ar(1., 0.2, 0.))
        }
    })
}
