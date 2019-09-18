extern crate rosc;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::time::Duration;
use std::{env, f32, thread};

fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).unwrap()
}

fn main() {
    let host_addr = "127.0.0.1:38122";
    let to_addr = "127.0.0.1:38042";
    let sock = UdpSocket::bind(host_addr).unwrap();

    let steps = 128;
    let step_size: f32 = 2.0 * f32::consts::PI / steps as f32;
    for i in 0.. {
        let x = 0.5 + (step_size * (i % steps) as f32).sin() / 2.0;
        let y = 0.5 + (step_size * (i % steps) as f32).cos() / 2.0;
        let mut msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/synth/sine".to_string(),
            args: Some(vec![OscType::Float(110. + (i as f32 * 2.)), OscType::Float(0.)]),
        })).unwrap();

        sock.send_to(&msg_buf, to_addr).unwrap();

        thread::sleep(Duration::from_millis(1000));
    }
}