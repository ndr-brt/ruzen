use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::str;
use std::sync::mpsc::Sender;
use crate::synth::Command;

pub struct UIServer {
    address: SocketAddrV4,
}

impl UIServer {
    pub fn new(address_string: &str) -> Self {
        UIServer {
            address: match SocketAddrV4::from_str(address_string) {
                Ok(address) => address,
                Err(err) => panic!(err),
            }
        }
    }

    pub fn listen(&self, command_out: Sender<Command>) {
        let sock = UdpSocket::bind(self.address).unwrap();
        println!("OSC server listening to {}", self.address);

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            let trimmed = message.trim();
                            println!("Received command from socket: {}", trimmed);

                            command_out.send(Command::Instrument(String::from(trimmed)));
                        },
                        Err(e) => println!("Message is not a string: {}", e)
                    }
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                }
            }
        }
    }
}