use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::{str, thread};
use rosc::{OscMessage, OscPacket, OscType};
use crate::{OSC_ADDRESS_CLIENT};
use std::fs::File;
use std::io::Read;
use crate::ui::interpreter::Interpreter;
use std::collections::HashMap;

mod interpreter;

pub struct UIServer {
    address: SocketAddrV4,
    osc_address_server: &'static str,
}

impl UIServer {
    pub fn new(ui_address_in: &str, osc_address_server: &'static str) -> Self {
        UIServer {
            osc_address_server,
            address: match SocketAddrV4::from_str(ui_address_in) {
                Ok(address) => address,
                Err(err) => panic!(err),
            }
        }
    }

    pub fn listen(&self) {
        let interpreter = Interpreter::new(self.osc_address_server);

        let code_sock = UdpSocket::bind(self.address).unwrap();
        println!("UI server listening on {}", self.address);

        let mut buf = [0u8; rosc::decoder::MTU];
        loop {
            match code_sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            println!("Received instruction:\n{}", message);
                            interpreter.run(message.to_string());
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