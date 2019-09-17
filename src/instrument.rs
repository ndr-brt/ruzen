use crate::oscillator::{Oscillator, Wave, Amplitude};
use crate::envelope::Envelope;
use crate::clock::{Hz, Clock};

pub struct Instrument {
    oscillator: Box<dyn Oscillator>,
    envelope: Envelope,
    amplitude: Amplitude,
    frequency_modulation: Box<dyn Oscillator>,
    phase: f64,
    clock: Clock,
}

impl Instrument {
    pub fn new(sample_rate: Hz, wave: Wave, frequency_modulation: Wave, phase: f64) -> Instrument {
        Instrument {
            oscillator: Oscillator::new(wave),
            envelope: Envelope::new(1., 1.),
            amplitude: Amplitude { min: -1., max: 1. },
            frequency_modulation: Oscillator::new(frequency_modulation),
            phase,
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