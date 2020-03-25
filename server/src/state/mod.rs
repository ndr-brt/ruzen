use crate::clock::{Hz, Clock};
use std::collections::HashMap;
use crate::instrument::{Instrument, kick, snare, catta, strange, sine, saw};
use crate::instrument::parameters::Parameters;
use crate::{Block, Sample};
use crate::state::ugen::{ValueAt};
use rosc::{OscMessage, OscPacket};
use crossbeam_channel::{Receiver, Sender};
use std::sync::Mutex;

pub mod ugen;

pub struct State {
    sample_rate: Hz,
    block_size: usize,
    definitions: HashMap<String, Box<fn(Parameters) -> Box<dyn ValueAt>>>,
    instruments: Mutex<HashMap<String, Instrument>>,
}

impl State {
    pub fn new(sample_rate: Hz, block_size: usize) -> State {
        State {
            sample_rate,
            block_size,
            instruments: Mutex::new(HashMap::new()),
            definitions: HashMap::new(),
        }
    }

    pub fn loop_forever(&self, osc_stream: Receiver<OscPacket>, signal_sink: Sender<Sample>) {
        loop {
            if let Ok(packet) = osc_stream.try_recv() {
                match packet {
                    OscPacket::Message(msg) => {
                        self.osc_message(msg);
                    }
                    OscPacket::Bundle(bundle) => {
                        println!("OSC Bundle: {:?}", bundle);
                    }
                }
            }

            let block = self.next_block();
            for sample in block {
                signal_sink.send(sample);
            }
        }
    }

    pub fn next_block(&self) -> Block {
        let mut instruments = self.instruments.lock().unwrap();

        // TODO: do retain not every block
        //instruments.retain(|_, instrument| !instrument.is_finished());

        let mut block = vec![];

        for i in 0..self.block_size {
            let sample = instruments.iter_mut().map(|(_, i)| {
                let time = i.tick();
                // TODO: is possible to interpolate?
                match self.definitions.get(i.name()) {
                    Some(definition) => definition(i.params()).value_at(time),
                    _ => 0.
                }
            }).sum();

            block.push(sample);
        }

        block
    }

    pub fn add(&mut self, name: &str, definition: fn(Parameters) -> Box<dyn ValueAt>) {
        self.definitions.insert(String::from(name), Box::new(definition));
    }

    pub fn instrument(&self, id: String, name: String, params: Parameters) {
        let mut instruments = self.instruments.lock().unwrap();
        match instruments.get_mut(id.as_str()) {
            Some(instrument) => {
                // TODO: is possible to interpolate?
                println!("Instrument with id {} already running", id);
                instrument.change_parameters(params)
            },
            None => {
                println!("Play new instrument: {}. params: {:?}", name, params);
                if self.definitions.contains_key(name.as_str()) {
                    let new_instrument = Instrument::new(name, Clock::new(self.sample_rate), params);
                    instruments.insert(id, new_instrument);
                } else {
                    println!("Instrument {} not known", name);
                }
            }
        }
    }

    pub fn osc_message(&self, msg: OscMessage) {
        println!("OSC message: {} {:?}", msg.addr, msg.args);
        let tokens: Vec<String> = msg.addr
            .split('/')
            .map(String::from)
            .collect();

        match tokens.get(1).map(|s| s.as_str()) {
            Some("hush") => self.hush(),
            Some("instrument") => {
                let name = tokens.get(2).unwrap();
                let id = tokens.last().unwrap();
                self.instrument(id.to_owned(), name.to_owned(), Parameters::from(msg.args));
            }
            any => { println!("OSC command {} not known", any.unwrap()) }
        }
    }

    pub fn hush(&self) {
        let mut instruments = self.instruments.lock().unwrap();
        instruments.retain(|_, _| false);
    }
}
