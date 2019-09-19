use std::sync::mpsc::{Sender, SendError};
use crate::synth::Command;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use rosc::{OscPacket};
use rosc::OscType::Float;
use std::error::Error;
use crate::command_factory::message_to_command;

pub struct OscServer {
    address: SocketAddrV4,
}

impl OscServer {
    pub fn new(address_string: &str) -> OscServer {
        OscServer {
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
                Ok((size, address)) => {
                    let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                    match message_to_command(packet) {
                        Ok(command) => { command_out.send(command); } ,
                        Err(err) => { println!("{}", err) }
                    }
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }
    }
}