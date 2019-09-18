extern crate rosc;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::time::Duration;
use std::{env, f32, thread};

const HOST_ADDRESS: &str = "127.0.0.1:38122";
const SERVER_ADDRESS: &str = "127.0.0.1:38042";


fn main() {
    let socket: UdpSocket = UdpSocket::bind(HOST_ADDRESS).unwrap();

    let steps = 128;
    let step_size: f32 = 2.0 * f32::consts::PI / steps as f32;
    for i in 0.. {
        let message = play(440. + ((i as f32)*4.));
        socket.send_to(&message, SERVER_ADDRESS);

        thread::sleep(Duration::from_millis(1000));
    }
}

fn play(frequency: f32) -> Vec<u8> {
    let mut msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/synth/sine".to_string(),
        args: Some(vec![OscType::Float(frequency), OscType::Float(0.)]),
    })).unwrap();
    msg_buf
}