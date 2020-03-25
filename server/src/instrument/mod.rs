pub mod parameters;

use crate::clock::{Clock};
use std::ops::Mul;
use self::parameters::{Parameters, GetParameter};
use crate::state::ugen::{UGen, ValueAt, SignalRange};
use crate::state::ugen::envelope::Envelope;
use crate::state::ugen::generator::Generator;
use crate::state::ugen::params::*;
use crate::Definition;

pub struct ParametersChanged {
    time: f64,
    params: Parameters,
}

pub struct Instrument {
    name: String,
    clock: Clock,
    params: Parameters,
    history: Vec<ParametersChanged>
}

impl Instrument {
    pub fn new(name: String, clock: Clock, params: Parameters) -> Self {
        Instrument {
            name,
            clock,
            params,
            history: vec![]
        }
    }

    pub fn sample(&mut self, definition: &Definition) -> f64 {
        let time = self.tick();

        // TODO: 0.1 is the constant for interpolation duration in seconds
        let definitions: Vec<f64> = self.history.iter()
            .filter(|h| h.time + 0.1 > time)
            .map(|h| definition(&h.params))
            .map(|h| h.value_at(time))
            .collect();

        (definitions.iter().sum::<f64>() + definition(&self.params).value_at(time)) / (definitions.len() + 1) as f64
    }

    pub fn tick(&mut self) -> f64 {
        self.clock.tick();
        self.clock.get()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn params(&self) -> Parameters {
        self.params.clone()
    }

    pub fn change_parameters(&mut self, params: Parameters) {
        self.history.push(ParametersChanged {
            time: self.clock.get(),
            params: self.params.clone(),
        });

        self.params = params;
    }

    pub(crate) fn is_finished(&self) -> bool {
        false
    }
}

// TODO: these functions should return just valueAt, can be Box::new be added outside?
pub(crate) fn kick(_params: &Parameters) -> Box<dyn ValueAt> {
    Box::new(Generator::sine()
        .frequency(Envelope::ar(0.0001, 1.5, -200.).range(45., 845.))
        .phase(UGen::from(1.0))
        * Envelope::line(1., 0., 1.)
    )
}

pub(crate) fn snare(_params: &Parameters) -> Box<dyn ValueAt> {
    Box::new(
    (Generator::sine().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.055, -4.).range(0., 0.25))
        + (Generator::sine().frequency(UGen::from(30.)) * Envelope::ar(0.0005, 0.075, -4.).range(0., 0.25))
        + (Generator::white_noise().mul(UGen::from(0.8)))
    )
}

pub(crate) fn strange(_params: &Parameters) -> Box<dyn ValueAt> {
    Box::new(
    Generator::saw().frequency(UGen::from(120.)) * UGen::from(0.5) +
        Generator::sine().frequency(UGen::from(100.)) * UGen::from(0.5)
    )
}

pub(crate) fn catta(_params: &Parameters) -> Box<dyn ValueAt> {
    let first_width_modulation = Generator::sine().frequency(UGen::from(5.)).range(0.1, 0.9);
    let second_width_modulation = Generator::sine().frequency(UGen::from(1.4)).range(0.1, 0.9);

    Box::new(
    Generator::pulse().frequency(UGen::from(234.)).width(first_width_modulation) * UGen::from(0.5) +
        Generator::pulse().frequency(UGen::from(215.)).width(second_width_modulation) * UGen::from(0.5)
    )
}

pub(crate) fn sine(params: &Parameters) -> Box<dyn ValueAt> {
    Box::new(
    Generator::sine()
        .frequency(params.get("freq", UGen::from(440.)))
        .phase(params.get("phase", UGen::from(0.)))
    )
}

pub(crate) fn saw(params: &Parameters) -> Box<dyn ValueAt> {
    Box::new(
    Generator::saw()
        .frequency(params.get("freq", UGen::from(440.)))
        .phase(params.get("phase", UGen::from(0.)))
    )
}

/*
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
        params,
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
        params,
        signal: Box::new(
            Generator::saw()
                .frequency(params.get("freq", UGen::from(440.)))
                .phase(params.get("phase", UGen::from(0.)))
        )
    })
}
*/
