use std::sync::mpsc::{Receiver, SyncSender};

use crate::clock::{Hz, Clock};
use crate::instrument::{snare, kick, Instrument, strange, catta, ContinuousInstrument, sine};
use std::collections::HashMap;
use std::net::UdpSocket;
use rosc::{OscPacket, OscType};
use crate::OSC_ADDRESS_SERVER;

pub type Parameters = HashMap<String, OscType>;

pub struct Synth {
    sample_rate: Hz,
}

impl Synth {
    pub fn new(sample_rate: Hz) -> Synth {
        Synth { sample_rate }
    }

    pub fn loop_forever(&self, osc_stream: Receiver<OscPacket>, signal_out: SyncSender<f64>) {
        let mut state = State::new(self.sample_rate);
        state.add("kick", |sample_rate, params| kick(sample_rate, params));
        state.add("snare", |sample_rate, params| snare(sample_rate, params));
        state.add("catta", |sample_rate, params| catta(sample_rate, params));
        state.add("strange", |sample_rate, params| strange(sample_rate, params));
        state.add("sine", |sample_rate, params| sine(sample_rate, params));

        loop {
            if let Ok(packet) = osc_stream.try_recv() {
                match packet {
                    OscPacket::Message(msg) => {
                        println!("OSC address: {}", msg.addr);
                        println!("OSC arguments: {:?}", msg.args);
                        let tokens: Vec<String> = msg.addr
                                .split('/')
                                .map(String::from)
                                .collect();

                        let param_list: Vec<String> = msg.args.iter()
                            .map(|t| t.to_owned())
                            .map(|t| t.string())
                            .map(|t| t.unwrap())
                            .collect();

                        // TODO: define a type for parameters
                        let mut params = HashMap::<String, OscType>::new();
                        let mut index = 0;
                        while index < param_list.len() {
                            let key: String = param_list.get(index).unwrap().clone();
                            let value = param_list.get(index + 1).unwrap().clone();
                            params.insert(key, OscType::Double(value.parse::<f64>().unwrap()));
                            index += 2;
                        }

                        let name = tokens.get(2).unwrap();
                        let id = tokens.last().unwrap();

                        state.instrument(id.to_owned(), name.to_owned(), params);
                    }
                    OscPacket::Bundle(bundle) => {
                        println!("OSC Bundle: {:?}", bundle);
                    }
                }
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
    instruments: HashMap<String, Box<dyn Instrument>>,
    definitions: HashMap<String, Box<dyn Fn(f64, Parameters) -> Box<dyn Instrument>>>,
}

impl State {
    pub fn new(sample_rate: Hz) -> State {
        State {
            sample_rate,
            instruments: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn next_sample(&mut self) -> f64 {
        self.instruments.retain(|_, instrument| !instrument.is_finished());

        self.instruments.iter_mut().map(|(_, i)| i.signal()).sum()
    }

    pub fn add(&mut self, name : &str, definition: fn(f64, Parameters) -> Box<dyn Instrument>) {
        self.definitions.insert(String::from(name), Box::new(definition));
    }

    pub fn instrument(&mut self, id: String, name: String, params: Parameters) {
        println!("Play new instrument: {}. params: {:?}", name, params);
        match self.definitions.get(name.as_str()) {
            Some(function) => { self.instruments.insert(id, function(self.sample_rate, params)); },
            None => println!("Instrument {} not known", name)
        }
    }
}
