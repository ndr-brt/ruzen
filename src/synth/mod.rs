use std::sync::mpsc::{Receiver, SyncSender};

use crate::clock::{Hz};
use crate::instrument::{snare, kick, Instrument};

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
    instruments: Vec<Box<dyn Instrument>>,
}

impl State {
    pub fn new(sample_rate: Hz) -> State {
        State {
            sample_rate,
            instruments: Vec::new()
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        self.instruments.retain(|i| !i.is_finished());

        self.instruments.iter_mut().map(|w| w.signal()).sum()
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::Instrument(name) => {
                match name.as_str() {
                    "kick" => self.instruments.push(kick(self.sample_rate)),
                    "snare" => self.instruments.push(snare(self.sample_rate)),
                    any => println!("Instrument {} not known", any)
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Instrument(String)
}
