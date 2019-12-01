use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::str;
use std::sync::mpsc::Sender;

pub struct GluonInterpreter {
    address: SocketAddrV4,
}

impl GluonInterpreter {
    pub fn new(address_string: &str) -> Self {
        GluonInterpreter {
            address: match SocketAddrV4::from_str(address_string) {
                Ok(address) => address,
                Err(err) => panic!(err),
            },
        }
    }

    pub fn listen(&self, gluon_out: Sender<String>) {
        let sock = UdpSocket::bind(self.address).unwrap();
        println!("Gluon Interpreter listening on {}", self.address);

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            let trimmed = message.trim();
                            println!("Received code chunk from socket: {}", trimmed);
                            match gluon_out.send(trimmed.to_string()) {
                                Ok(_) => println!("Message sent"),
                                Err(e) => println!("Error sending code chunk {}", e)
                            }
                        },
                        Err(e) => println!("Code chunk is not a string: {}", e)
                    }
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                }
            }
        }
    }
}