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
    let cycle = 1000;
    loop {
        pattern("kick kick snare ~ kick kick snare ~ snare", rrand(400., 1900.) as usize);
    }
}

fn pattern(pattern: &str, cycle_time: usize) {
    let tokens: Vec<&str> = pattern.split(" ").collect();
    let time_each: usize = cycle_time / tokens.len();
    tokens.iter().for_each(|token| {
        if *token != "~" {
            instrument(token).play();
        }
        sleep(time_each as u64);
    })
}

#[derive(Debug, Clone, Copy)]
struct Synth<'a> {
    name: &'a str,
    frequency: Hz,
    phase: f32,
    attack: f64,
    release: f64,
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

fn instrument(name: &str) -> Instrument {
    Instrument::new(name)
}

fn send_osc_message(msg_buf: Vec<u8>) {
    UdpSocket::bind(HOST_ADDRESS).unwrap().send_to(&msg_buf, SERVER_ADDRESS);
}