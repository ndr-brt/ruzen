use std::sync::mpsc::Sender;
use crate::synth::Command;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use rosc::{OscPacket};
use crate::oscillator::Wave;
use rosc::OscType::Float;
use std::error::Error;

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
                    println!("Received packet with size {} from: {}", size, address);
                    let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                    match self.handle_packet(packet) {
                        Ok(command) => {
                            command_out.send(command);
                        },
                        Err(err) => println!("{}", err)
                    }
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }
    }

    fn handle_packet(&self, packet: OscPacket) -> Result<Command, Box<dyn Error>> {
        match packet {
            OscPacket::Message(msg) => {
                println!("OSC address: {}", msg.addr);
                let wave = match msg.addr.split('/').last() {
                    Some("sine") => Wave::Sine,
                    Some("saw") => Wave::Saw,
                    Some(_) => {
                        println!("instrument not found, default is sine");
                        Wave::Sine
                    }
                    None => {
                        println!("instrument not found, default is sine");
                        Wave::Sine
                    }
                };

                match msg.args {
                    Some(args) => {
                        println!("OSC arguments: {:?}", args);
                        match args[0] {
                            Float(frequency) => {
                                Result::Ok(Command::Play(wave(frequency as f64, 0.), Wave::None, 0.))
                            }
                            _ => {
                                Result::Err(Box::from("Not a valid frequency"))
                            }
                        }

                    }
                    None => {
                        Result::Err(Box::from("No arguments in message."))
                    },
                }
            }
            OscPacket::Bundle(bundle) => {
                Result::Err(Box::from(format!("OSC Bundle: {:?}", bundle)))
            }
        }
    }

}