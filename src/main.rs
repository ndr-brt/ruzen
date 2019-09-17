extern crate cpal;
extern crate failure;
extern crate rand;

use std::sync::mpsc::{sync_channel, channel};
use std::thread;

use rand::Rng;

use crate::out::Out;
use std::thread::sleep;
use std::time::Duration;
use crate::synth::{Synth, Command, Wave};

mod clock;
mod envelope;
mod out;
mod synth;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();
    let synth = Synth::new(sample_rate);

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || synth.loop_forever(cmd_in, sig_out));

    let mut rng = rand::thread_rng();
    loop {
        sleep(Duration::from_millis(rng.gen_range(500, 1500)));
        let frequency: f64 = rng.gen_range(110.0, 880.0);
        let phase: f64 = rng.gen_range(0., 3.14);
        let command = Command::Play(Wave::Saw, sample_rate, frequency, phase);
        match cmd_out.send(command) {
            Ok(_) => println!("Sent new sine with frequency {}", frequency),
            Err(err) => println!("Error sending command {}", err),
        };
    }
}