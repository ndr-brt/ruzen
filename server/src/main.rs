extern crate cpal;
extern crate failure;
extern crate rand;
extern crate rlua;
extern crate rosc;
extern crate crossbeam_channel;

use std::thread;

use crate::out::Out;
use crate::synth::Synth;
use crate::ui::UIServer;
use rosc::OscPacket;
use crate::osc_server::OscServer;
use crossbeam_channel::{unbounded, bounded};

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

type Block = Vec<f64>;

fn main() {
    let block_size: u16 = 128;
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let synth = Synth::new(out.sample_rate(), 128);
    let osc_server = OscServer::new(OSC_ADDRESS_SERVER);

    let (osc_sink, osc_stream) = unbounded::<OscPacket>();
    let (signal_sink, signal_stream) = bounded::<Block>(out.buffer_size());

    thread::spawn(move || out.loop_forever(signal_stream));
    thread::spawn(move || synth.loop_forever(osc_stream, signal_sink));
    thread::spawn(move || osc_server.listen_forever(osc_sink));

    let ui_server = UIServer::new(UI_ADDRESS_IN, OSC_ADDRESS_SERVER);
    ui_server.listen();
}