use crate::oscillator::{Oscillator, Wave, Amplitude};
use crate::envelope::Envelope;
use crate::clock::{Hz, Clock};

pub struct Instrument {
    oscillator: Box<dyn Oscillator>,
    envelope: Envelope,
    amplitude: Amplitude,
    frequency: Box<dyn Oscillator>,
    phase: f64,
    clock: Clock,
}

impl Instrument {
    pub fn new(sample_rate: Hz, wave: Wave, frequency: Wave, phase: f64) -> Instrument {
        Instrument {
            oscillator: Oscillator::new(wave),
            envelope: Envelope::new(1., 1.),
            amplitude: Amplitude { min: -1., max: 1. },
            frequency: Oscillator::new(frequency),
            phase,
            clock: Clock::new(sample_rate),
        }
    }

    pub fn signal(&mut self) -> f64 {
        self.clock.tick();
        let x = self.frequency.signal(self.clock.get(), 0., 0.);
        let signal = self.oscillator.signal(self.clock.get(), x, self.phase);
        self.envelope.apply(self.clock.get(), signal)
    }
}