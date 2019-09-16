use crate::clock::{Hz, Clock};
use crate::envelope::Envelope;
use std::sync::mpsc::{SyncSender, Receiver};
use std::f64::consts::PI;

pub struct Synth {
    sample_rate: Hz,
}

impl Synth {
    pub fn new(sample_rate: Hz) -> Synth {
        Synth { sample_rate }
    }

    pub fn loop_forever(&self, command_in: Receiver<f64>, signal_out: SyncSender<f64>) {
        let mut state = State::new(self.sample_rate);
        loop {
            if let Ok(frequency) = command_in.try_recv() {
                state.add_sine(frequency);
            }

            let result = signal_out.send(state.next_sample());
            match result {
                Ok(_data) => (),
                Err(err) => println!("Error: {}", err)
            }
        }
    }

}

pub struct State {
    sample_rate: Hz,
    waves: Vec<Sine>,
}

impl State {
    pub fn new(sample_rate: Hz) -> State {
        State {
            sample_rate,
            waves: Vec::new() // TODO: free the finished synths!
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        self.waves.iter_mut().map(|w| w.signal()).sum()
    }

    pub fn add_sine(&mut self, frequency: f64) {
        let sine = Sine::new(self.sample_rate, frequency).envelope(Envelope::new(1., 1.));
        self.waves.push(sine);
    }
}

#[derive(Clone,Copy)]
pub struct Sine{
    frequency: f64,
    clock: Clock,
    envelope: Envelope,
}

impl Sine {
    pub fn new(sample_rate: Hz, frequency: Hz) -> Sine {
        Sine {
            frequency,
            clock: Clock::new(sample_rate),
            envelope: Envelope::new(0.0, 0.0)
        }
    }

    pub fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = (self.clock.get() * self.frequency * 2.0 * PI).sin();
        if self.envelope.is_valid() {
            self.envelope.apply(self.clock.get(), signal)
        } else {
            signal
        }
    }

    pub fn envelope(&mut self, envelope: Envelope) -> Self {
        self.envelope = envelope;
        self.clone()
    }
}
