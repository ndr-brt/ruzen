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

    pub fn loop_forever(&self, command_in: Receiver<Command>, signal_out: SyncSender<f64>) {
        let mut state = State::new();
        loop {
            if let Ok(command) = command_in.try_recv() {
                state.interpret(command);
            }

            let result = signal_out.send(state.next_sample());
            match result {
                Ok(_data) => (),
                Err(err) => println!("Error: {}", err)
            }
        }
    }

}

pub enum Wave { Sine, Saw }

pub trait Oscillator {
    fn signal(&self, time: f64, frequency: Hz) -> f64;
}

impl dyn Oscillator {
    pub fn new(wave: Wave) -> Box<dyn Oscillator> {
        match wave {
            Wave::Sine => Box::new(Sine),
            Wave::Saw => Box::new(Saw),
        }
    }
}

pub struct Sine;
impl Oscillator for Sine {
    fn signal(&self, time: f64, frequency: Hz) -> f64 {
        (time * frequency * 2.0 * PI).sin()
    }
}

pub struct Saw;
impl Oscillator for Saw {
    fn signal(&self, time: f64, frequency: Hz) -> f64 {
        ((time * frequency) % 1.)
    }
}

pub struct Instrument {
    oscillator: Box<dyn Oscillator>,
    envelope: Envelope,
    frequency: Hz,
    clock: Clock,
}


impl Instrument {
    pub fn new(sample_rate: Hz, wave: Wave, frequency: Hz) -> Instrument {
        Instrument {
            oscillator: Oscillator::new(wave),
            envelope: Envelope::new(1., 1.),
            frequency,
            clock: Clock::new(sample_rate),
        }
    }

    pub fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = self.oscillator.signal(self.clock.get(), self.frequency);
        self.envelope.apply(self.clock.get(), signal)
    }
}

pub struct State {
    instruments: Vec<Instrument>,
}

impl State {
    pub fn new() -> State {
        State {
            instruments: Vec::new() // TODO: free the finished instruments!
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        self.instruments.iter_mut().map(|w| w.signal()).sum()
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::Play(wave, sample_rate, frequency) => {
                self.instruments.push(Instrument::new(sample_rate, wave, frequency));
            }
        }
    }
}

pub enum Command {
    Play(Wave, Hz, Hz)
}
