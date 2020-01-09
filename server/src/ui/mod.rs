extern crate rhai;
extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::{str, thread};
use std::sync::mpsc::{Sender, channel};
use crate::synth::Command;
use std::thread::sleep;
use std::time::Duration;
use std::ops::Div;
use rhai::{Engine, Scope};
use rhai::Any;
use rhai::RegisterFn;
use std::ffi::FromBytesWithNulError;
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use crate::OSC_ADDRESS_CLIENT;

const CYCLE_TIME: Duration = Duration::from_secs(1);

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
            let mut buf = [0u8; rosc::decoder::MTU];

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

    fn sine(&mut self) {
        println!("SINE!");
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/new/sine".to_string(),
            args: vec![],
        })).unwrap();

        self.sender.send(msg_buf);
    }

    fn inst(&mut self, name: String) {
        println!("Instrument: {}", name);
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/instrument/".to_owned() + &name, // TODO: use string format
            args: vec![],
        })).unwrap();

        self.sender.send(msg_buf);
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

        let mut engine = Engine::new();
        let mut scope = Scope::new();
        let interpreter = Interpreter::new(self.osc_address_server);
        scope.push(("r".to_string(), Box::new(interpreter)));

        engine.register_type::<Interpreter>();
        engine.register_fn("sine", Interpreter::sine);
        engine.register_fn("inst", Interpreter::inst);

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match code_sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            let trimmed = message.trim();
                            println!("Received instruction:\n{}", trimmed);
                            match engine.eval_with_scope::<()>(&mut scope, trimmed) {
                                Ok(result) => println!("Code evaluated correctly"),
                                Err(e) => println!("Error: {}", e.to_string())
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