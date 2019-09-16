extern crate cpal;
extern crate failure;
extern crate rand;

use std::f64::consts::PI;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

use rand::Rng;

use crate::clock::Clock;
use crate::clock::Hz;
use crate::envelope::Envelope;
use crate::out::Out;

mod clock;
mod envelope;
mod out;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || play(sample_rate, sig_out));

    loop { }
}

fn play(sample_rate: Hz, sig_out: SyncSender<f64>) -> () {
    let mut rng = rand::thread_rng();
    loop {
        let frequency: f64 = rng.gen_range(220.0, 440.0);
        println!("Frequency {}", frequency);
        let mut sine = Sine::new(sample_rate, frequency).envelope(Envelope::new(1., 1.));
        while !sine.is_finished() {

            let result = sig_out.send(sine.signal());
            match result {
                Ok(_data) => (),
                Err(err) => println!("{}", err)
            }
        };
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
