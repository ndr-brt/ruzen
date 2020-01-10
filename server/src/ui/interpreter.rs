use std::sync::mpsc::{Sender, channel};
use rosc::{OscMessage, OscPacket, OscType};
use std::{str, thread};
use std::net::UdpSocket;
use crate::OSC_ADDRESS_CLIENT;
use super::rosc::encoder;

#[derive(Clone)]
pub struct Interpreter {
    osc_sink: Sender<OscPacket>,
}

impl Interpreter {
    pub(crate) fn new(osc_address_out: &'static str) -> Interpreter {
        let (osc_sink, osc_stream) = channel::<OscPacket>();

        thread::spawn(move || {
            let socket = UdpSocket::bind(OSC_ADDRESS_CLIENT).unwrap();
            let mut buf = [0u8; rosc::decoder::MTU];

            loop {
                match osc_stream.recv() {
                    Ok(osc) => {
                        match socket.send_to(encoder::encode(&osc).unwrap().as_slice(), osc_address_out) {
                            Ok(size) => println!("Sent {} osc bytes to server", size),
                            Err(e) => println!("Error sending osc message to server: {}", e.to_string())
                        }
                    },
                    Err(e) => println!("Some error receiving osc message to send: {}", e.to_string())
                }
            }
        });

        Interpreter { osc_sink }
    }

    pub(crate) fn sender(&self) -> Sender<OscPacket> {
        self.osc_sink.clone()
    }

}