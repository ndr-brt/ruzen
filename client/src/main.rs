extern crate rosc;
extern crate rand;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{UdpSocket};
use std::time::Duration;
use std::{f32};
use rand::thread_rng;
use crate::rand::Rng;

type Hz = f64;

const HOST_ADDRESS: &str = "127.0.0.1:38122";
const SERVER_ADDRESS: &str = "127.0.0.1:38042";

fn main() {
    let steps = 128;
    let step_size: f32 = 2.0 * f32::consts::PI / steps as f32;
    for i in 0.. {

        /*synth("sine").freq(rrand(330., 550.) as f32).play();

        sleep(700);*/

        /*synth("saw").freq(rrand(330., 550.)).play();

        sleep(700);*/

        instrument("kick").play();

        sleep(1000);
    }
}

#[derive(Debug, Clone, Copy)]
struct Synth<'a> {
    name: &'a str,
    frequency: Hz,
    phase: f32,
    attack: f64,
    release: f64,
}

impl Synth<'_> {
    pub fn new(name: &str) -> Synth {
        Synth {
            name,
            frequency: 440.0,
            phase: 0.0,
            attack: 1.0,
            release: 1.0
        }
    }

    pub fn freq(&mut self, frequency: Hz) -> Synth {
        self.frequency = frequency;
        *self
    }

    pub fn play(&self) {
        let mut msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: format!("/synth/{}", self.name),
            args: Some(vec![OscType::Double(self.frequency), OscType::Float(0.)]),
        })).unwrap();

        send_osc_message(msg_buf)
    }
}

struct Instrument<'a> {
    name: &'a str
}
impl Instrument<'_> {
    pub fn new(name: &str) -> Instrument {
        Instrument {
            name
        }
    }

    pub fn play(&self) {
        let mut msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: format!("/instrument/{}", self.name),
            args: Some(vec![]),
        })).unwrap();

        send_osc_message(msg_buf)
    }
}

fn sleep(time: u64) {
    std::thread::sleep(Duration::from_millis(time));
}

fn rrand(from: f64, to: f64) -> f64 {
    thread_rng().gen_range::<f64, f64, f64>(from, to)
}

fn synth(name: &str) -> Synth {
    Synth::new(name)
}

fn instrument(name: &str) -> Instrument {
    Instrument::new(name)
}

fn send_osc_message(msg_buf: Vec<u8>) {
    UdpSocket::bind(HOST_ADDRESS).unwrap().send_to(&msg_buf, SERVER_ADDRESS);
}