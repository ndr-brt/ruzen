use crate::clock::{Hz, Clock};
use std::sync::mpsc::{SyncSender, Receiver};
use crate::oscillator::{Wave};
use crate::instrument::{Play, Instruments, Sine, Kick, sine, saw, kick, snare};
use crate::synth::Command::Instrument;

pub struct Synth {
    sample_rate: Hz,
}

impl Synth {
    pub fn new(sample_rate: Hz) -> Synth {
        Synth { sample_rate }
    }

    pub fn loop_forever(&self, command_in: Receiver<Command>, signal_out: SyncSender<f64>) {
        let mut state = State::new(self.sample_rate);
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

pub struct State {
    sample_rate: Hz,
    instruments: Vec<Box<dyn Play>>,
}

impl State {
    pub fn new(sample_rate: Hz) -> State {
        State {
            sample_rate,
            instruments: Vec::new() // TODO: free the finished instruments!
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        self.instruments.iter_mut().map(|w| w.signal()).sum()
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::Play(wave, frequency, phase) => {
                match wave {
                    Wave::Sine(frequency, phase) => self.instruments.push(Box::new(sine(self.sample_rate, frequency))),
                    Wave::Saw(frequency, phase) => self.instruments.push(Box::new(saw(self.sample_rate, frequency))),
                    _ => {}
                }
            }
            Command::Instrument(name) => {
                match name {
                    Instruments::Kick => self.instruments.push(Box::new(kick(self.sample_rate))),
                    Instruments::Snare => self.instruments.push(Box::new(snare(self.sample_rate))),
                }

            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Play(Wave, Wave, Hz),
    Instrument(Instruments)
}
