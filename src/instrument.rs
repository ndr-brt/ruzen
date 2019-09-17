use crate::oscillator::{Oscillator, Wave};
use crate::envelope::Envelope;
use crate::clock::{Hz, Clock};

pub struct Instrument {
    oscillator: Box<dyn Oscillator>,
    envelope: Envelope,
    frequency: Hz,
    phase: f64,
    clock: Clock,
}

impl Instrument {
    pub fn new(sample_rate: Hz, wave: Wave, frequency: Hz, phase: f64) -> Instrument {
        Instrument {
            oscillator: Oscillator::new(wave),
            envelope: Envelope::new(1., 1.),
            frequency,
            phase,
            clock: Clock::new(sample_rate),
        }
    }

    pub fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = self.oscillator.signal(self.clock.get(), self.frequency, self.phase);
        self.envelope.apply(self.clock.get(), signal)
    }
}