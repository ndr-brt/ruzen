extern crate cpal;
extern crate failure;
extern crate rand;
extern crate rlua;
extern crate rosc;
#[macro_use]
extern crate crossbeam_channel;

use std::sync::mpsc::{sync_channel, channel};
use std::thread;

use crate::out::Out;
use crate::synth::Synth;
use crate::ui::UIServer;
use rosc::OscPacket;
use std::net::UdpSocket;
use crate::osc_server::OscServer;

mod clock;
mod out;
mod plot;
mod instrument;
mod synth;
mod ui;
mod osc_server;

const OSC_ADDRESS_SERVER: &str = "127.0.0.1:38041";
const OSC_ADDRESS_CLIENT: &str = "127.0.0.1:38042";
const UI_ADDRESS_IN: &str = "127.0.0.1:38043";

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (osc_sink, osc_stream) = channel::<OscPacket>();
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();
    let synth = Synth::new(sample_rate);
    let osc_server = OscServer::new(OSC_ADDRESS_SERVER);

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || synth.loop_forever(osc_stream, sig_out));
    thread::spawn(move || osc_server.listen_forever(osc_sink));

    let ui_server = UIServer::new(UI_ADDRESS_IN, OSC_ADDRESS_SERVER);
    ui_server.listen();
}