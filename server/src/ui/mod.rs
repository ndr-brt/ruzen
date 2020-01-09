extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::{str, thread};
use std::sync::mpsc::{Sender, channel};
use crate::synth::Command;
use std::thread::sleep;
use std::time::Duration;
use std::ops::Div;
use dyon::{error, run, Module, Dfn, load};
use std::ffi::FromBytesWithNulError;
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use crate::{OSC_ADDRESS_CLIENT, OSC_ADDRESS_SERVER};
use dyon::{run_str, Runtime, load_str};
use std::sync::Arc;
use dyon::Type;

pub struct UIServer {
    address: SocketAddrV4,
    osc_address_server: &'static str,
}

#[derive(Clone)]
struct Interpreter {
    sender: Sender<Vec<u8>>,
}

impl Interpreter {
    fn new(osc_address_out: &'static str) -> Interpreter {
        let (sender, receiver) = channel::<Vec<u8>>();

        thread::spawn(move || {
            let socket = UdpSocket::bind(OSC_ADDRESS_CLIENT).unwrap();

            loop {
                match receiver.recv() {
                    Ok(osc) => {
                        match socket.send_to(osc.as_slice(), osc_address_out) {
                            Ok(size) => println!("Sent {} osc bytes to server", size),
                            Err(e) => println!("Error sending osc message to server: {}", e.to_string())
                        }
                    },
                    Err(e) => println!("Some error receiving osc message to send: {}", e.to_string())
                }
            }
        });

        Interpreter { sender }
    }

    fn inst(&mut self, name: String) {
        println!("Instrument: {}", name);
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/instrument/".to_owned() + &name, // TODO: use string format
            args: vec![],
        })).unwrap();

        match self.sender.send(msg_buf) {
            Ok(_) => {},
            Err(_) => {}
        };
    }

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
        let code_sock = UdpSocket::bind(self.address).unwrap();
        println!("UI server listening on {}", self.address);

        let mut buf = [0u8; rosc::decoder::MTU];
        let interpreter = Interpreter::new(OSC_ADDRESS_SERVER);

        loop {
            match code_sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            let trimmed = message.trim();
                            println!("Received instruction:\n{}", trimmed);
                            let mut module= Module::new();
                            module.add_str("say_hello", say_hello, Dfn::nl(vec![], Type::Void));
                            load_str("main.dyon", Arc::new(format!(r#"
                                fn main() {{
                                    {}
                                }}
                            "#, trimmed).into()), &mut module);
                            let mut runtime = Runtime::new();
                            runtime.run(&Arc::new(module));
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

dyon_fn!{fn say_hello() {
    println!("hi!");
}}