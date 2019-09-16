extern crate cpal;
extern crate failure;
extern crate rand;

use std::f64::consts::PI;
use std::sync::mpsc::{sync_channel, SyncSender, channel, Receiver};
use std::thread;

use rand::Rng;

use crate::clock::Clock;
use crate::clock::Hz;
use crate::envelope::Envelope;
use crate::out::Out;
use std::thread::sleep;
use std::time::Duration;

mod clock;
mod envelope;
mod out;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (cmd_out, cmd_in) = channel::<f64>();
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || play(sample_rate, sig_out, cmd_in));

    let mut rng = rand::thread_rng();
    loop {
        sleep(Duration::from_millis(rng.gen_range(500, 1500)));
        let frequency: f64 = rng.gen_range(110.0, 880.0);
        match cmd_out.send(frequency) {
            Ok(_) => println!("Sent new sine with frequency {}", frequency),
            Err(err) => println!("Error sending command {}", err),
        };
    }
}

fn play(sample_rate: Hz, sig_out: SyncSender<f64>, cmd_in: Receiver<f64>) -> () {
    let mut state = State::new(sample_rate);
    loop {
        if let Ok(frequency) = cmd_in.try_recv() {
            state.add_sine(frequency);
        }

        let result = sig_out.send(state.next_sample());
        match result {
            Ok(_data) => (),
            Err(err) => println!("Error: {}", err)
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
            waves: Vec::new()
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

    pub fn is_finished(&self) -> bool {
        self.envelope.is_valid() && self.envelope.duration() < self.clock.get()
    }
}
