use std::sync::mpsc::Sender;
use crate::synth::Command;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use rosc::{OscPacket, OscType};
use crate::oscillator::Wave;
use rosc::OscType::Float;
use std::error::Error;

pub struct OscServer {
    address: SocketAddrV4,
}

impl OscServer {
    pub fn new(address_string: &str) -> OscServer {
        let address = match SocketAddrV4::from_str(address_string) {
            Ok(addr) => addr,
            Err(err) => panic!(err),
        };
        OscServer { address }
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
                match msg.args {
                    Some(args) => {
                        println!("OSC arguments: {:?}", args);
                        match args[0] {
                            Float(frequency) => {
                                Result::Ok(Command::Play(Wave::Sine(frequency as f64, 0.0), Wave::None, 0.))
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

/*    let mut rng = rand::thread_rng();
    loop {
        //sleep(Duration::from_millis(rng.gen_range(500, 1500)));
        sleep(Duration::from_millis(2000));
        let frequency: f64 = rng.gen_range(110.0, 440.0);
        let phase: f64 = rng.gen_range(0., 3.14);
        let command = Command::Play(Wave::Sine(frequency, phase), Wave::Sine(rng.gen_range(0., 10.), 1.), phase);
        match cmd_out.send(command) {
            Ok(_) => println!("Sent new sine with frequency {}", frequency),
            Err(err) => println!("Error sending command {}", err),
        };
    }*/