use std::sync::mpsc::{Receiver, SyncSender};

use crate::clock::{Hz, Clock};
use crate::instrument::{snare, kick, Instrument, strange, catta, ContinuousInstrument, sine};
use std::collections::HashMap;
use std::net::UdpSocket;
use rosc::OscPacket;
use crate::OSC_ADDRESS_SERVER;

pub struct Synth {
    sample_rate: Hz,
}

impl Synth {
    pub fn new(sample_rate: Hz) -> Synth {
        Synth { sample_rate }
    }

    pub fn loop_forever(&self, osc_stream: Receiver<OscPacket>, signal_out: SyncSender<f64>) {
        let mut state = State::new(self.sample_rate);

        loop {
            if let Ok(packet) = osc_stream.try_recv() {
                match packet {
                    OscPacket::Message(msg) => {
                        println!("OSC address: {}", msg.addr);
                        println!("OSC arguments: {:?}", msg.args);
                        state.play();
                    }
                    OscPacket::Bundle(bundle) => {
                        println!("OSC Bundle: {:?}", bundle);
                        state.play();
                    }
                }
            }

            let result = signal_out.send(state.next_sample());
            match result {
                Ok(_data) => (),
                Err(err) => println!("Error: {}", err)
            }
        }

//        let mut state = State::new(self.sample_rate);
//        state.add("kick", |sample_rate| kick(sample_rate));
//        state.add("snare", |sample_rate| snare(sample_rate));
//        state.add("catta", |sample_rate| catta(sample_rate));
//        state.add("strange", |sample_rate| strange(sample_rate));
//        loop {
//            if let Ok(command) = command_in.try_recv() {
//                state.interpret(command);
//            }
//
//            let result = signal_out.send(state.next_sample());
//            match result {
//                Ok(_data) => (),
//                Err(err) => println!("Error: {}", err)
//            }
//        }
    }

}

pub struct State {
    sample_rate: Hz,
    instruments: Vec<Box<dyn Instrument>>,
    definitions: HashMap<String, Box<dyn Fn(f64) -> Box<dyn Instrument>>>,
}

impl State {
    pub fn new(sample_rate: Hz) -> State {
        State {
            sample_rate,
            instruments: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        self.instruments.retain(|i| !i.is_finished());

        self.instruments.iter_mut().map(|w| w.signal()).sum()
    }

    pub fn add(&mut self, name : &str, definition: fn(f64) -> Box<dyn Instrument>) {
        self.definitions.insert(String::from(name), Box::new(definition));
    }

    pub fn play(&mut self) {
        println!("Add new instrument");
        self.instruments.push(sine (self.sample_rate));
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::Instrument(name) => {
                match self.definitions.get(name.as_str()) {
                    Some(function) => self.instruments.push(function(self.sample_rate)),
                    None => println!("Instrument {} not known", name)
                }
            },
            Command::Wave(name) => {
                self.instruments.push(sine(self.sample_rate))
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Instrument(String),
    Wave(String)
}
