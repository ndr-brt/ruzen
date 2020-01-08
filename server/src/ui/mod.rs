extern crate rhai;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::{str, thread};
use std::sync::mpsc::{Sender, channel};
use crate::synth::Command;
use std::thread::sleep;
use std::time::Duration;
use std::ops::Div;
use rhai::{Engine, Scope};
use self::rhai::Any;
use self::rhai::RegisterFn;
use std::ffi::FromBytesWithNulError;

const CYCLE_TIME: Duration = Duration::from_secs(1);

pub struct UIServer {
    address: SocketAddrV4,
    command_out: Sender<Command>,
}

#[derive(Clone)]
struct Interpreter {
    command_out: Sender<Command>
}

impl Interpreter {
    fn new(command_out: Sender<Command>) -> Interpreter {
        Interpreter { command_out }
    }

    fn sine(&mut self) {
        println!("SINE!");
        self.command_out.send(Command::Wave("sine".to_string()));
    }

}

impl UIServer {
    pub fn new(address_string: &str, command_out: Sender<Command>) -> Self {
        UIServer {
            command_out,
            address: match SocketAddrV4::from_str(address_string) {
                Ok(address) => address,
                Err(err) => panic!(err),
            }
        }
    }

    pub fn listen(&self) {
        let sock = UdpSocket::bind(self.address).unwrap();
        println!("UI server listening on {}", self.address);

        let mut engine = Engine::new();
        let mut scope = Scope::new();
        let interpreter = Interpreter::new(self.command_out.clone());
        scope.push(("r".to_string(), Box::new(interpreter)));

        engine.register_type::<Interpreter>();
        engine.register_fn("sine", Interpreter::sine);

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match sock.recv_from(&mut buf) {
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