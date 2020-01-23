use crate::clock::{Hz};
use crate::instrument::{snare, kick, strange, catta, sine};
use crate::instrument::parameters::Parameters;
use rosc::{OscPacket};
use crate::synth::state::State;
use crossbeam_channel::{Receiver, Sender};

pub mod ugen;
mod state;

pub struct Synth {
    sample_rate: Hz,
}

impl Synth {
    pub fn new(sample_rate: Hz) -> Synth {
        Synth { sample_rate }
    }

    pub fn loop_forever(&self, osc_stream: Receiver<OscPacket>, signal_out: Sender<f64>) {
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

                        match tokens.get(1).map(|s| s.as_str()) {
                            Some("hush") => state.hush(),
                            Some("instrument") => {
                                let name = tokens.get(2).unwrap();
                                let id = tokens.last().unwrap();
                                state.instrument(id.to_owned(), name.to_owned(), Parameters::from(msg.args));
                            }
                            any => { println!("OSC command {} not known", any.unwrap()) }
                        }

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
