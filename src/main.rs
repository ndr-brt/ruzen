extern crate cpal;
extern crate failure;
extern crate rand;

use std::sync::mpsc::{sync_channel, channel};
use std::thread;

use crate::out::Out;
use crate::synth::{Synth, Command };
use crate::osc_server::OscServer;

mod clock;
mod out;
mod plot;
mod osc_server;
mod command_factory;
mod instrument;
mod synth;
mod ugen;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();
    let synth = Synth::new(sample_rate);

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || synth.loop_forever(cmd_in, sig_out));

    let osc_server = OscServer::new("127.0.0.1:38042");
    osc_server.listen(cmd_out);
}