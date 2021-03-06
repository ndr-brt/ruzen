use crate::clock::Hz;
use std::collections::HashMap;
use crate::instrument::Instrument;
use crate::instrument::parameters::Parameters;
use crate::Block;

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

    pub fn next_block(&mut self) -> Block {
        self.instruments.retain(|_, instrument| !instrument.is_finished());

        let sample = self.instruments.iter_mut().map(|(_, i)| i.signal()).sum();

        vec![sample]
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

    pub fn hush(&mut self) {
        self.instruments.retain(|_, _| false);
    }
}
