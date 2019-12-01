extern crate cpal;
extern crate failure;
extern crate rand;

use std::sync::mpsc::{sync_channel, channel};
use std::thread;

use crate::out::Out;
use crate::synth::{Synth, Command };
use crate::osc_server::OscServer;
use crate::ui::UIServer;

mod clock;
mod out;
mod plot;
mod osc_server;
mod command_factory;
mod instrument;
mod synth;
mod ugen;
mod ui;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();
    let synth = Synth::new(sample_rate);
    let osc_server = OscServer::new("127.0.0.1:38042");
    let osc_sender = cmd_out.clone();

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || synth.loop_forever(cmd_in, sig_out));

    osc_server.listen(cmd_out);
}