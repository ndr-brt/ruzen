extern crate cpal;
extern crate failure;
extern crate rand;

use std::sync::mpsc::{sync_channel, channel};
use std::thread;

use rand::Rng;

use crate::out::Out;
use std::thread::sleep;
use std::time::Duration;
use crate::synth::{Synth, Command };
use crate::oscillator::Wave;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use rosc::OscPacket;
use crate::osc_server::OscServer;

mod clock;
mod envelope;
mod out;
mod osc_server;
mod instrument;
mod oscillator;
mod synth;

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
