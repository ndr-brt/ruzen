use crate::oscillator::{ Oscillator, Wave };
use crate::envelope::Envelope;
use crate::clock::{Hz, Clock};

#[derive(PartialEq, Debug)]
pub enum Instruments {
    Kick
}

pub struct Instrument {
    oscillator: Box<dyn Oscillator>,
    envelope: Envelope,
    frequency_modulation: Box<dyn Oscillator>,
    phase: f64,
    clock: Clock,
}

impl Instrument {
    pub fn new(sample_rate: Hz, wave: Wave, frequency_modulation: Wave, phase: f64) -> Instrument {
        Instrument {
            oscillator: Oscillator::new(wave),
            envelope: Envelope::new(0.005, 1.),
            frequency_modulation: Oscillator::new(frequency_modulation),
            phase,
            clock: Clock::new(sample_rate),
        }
    }

    pub fn kick(sample_rate: Hz) -> Instrument {
        Instrument {
            oscillator: Oscillator::new(Wave::Sine(60.0, 0.)),
            envelope: Envelope::new(0.005, 1.),
            frequency_modulation: Oscillator::new(Wave::Line(1.,0.,1.)),
            phase: 0.,
            clock: Clock::new(sample_rate),
        }
    }

    pub fn signal(&mut self) -> f64 {
        self.clock.tick();
        let frequency_scale = self.frequency_modulation.signal(self.clock.get(), self.frequency_modulation.frequency(), 0.);
        let modulated_frequency = self.frequency() + (self.frequency() * frequency_scale);
        let signal = self.oscillator.signal(self.clock.get(), modulated_frequency, self.phase);
        self.envelope.apply(self.clock.get(), signal)
    }

    pub fn frequency(&self) -> Hz {
        self.oscillator.frequency()
    }
}