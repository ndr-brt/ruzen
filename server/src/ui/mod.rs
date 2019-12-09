use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::{str, thread};
use std::sync::mpsc::{Sender, channel};
use crate::synth::Command;
use std::thread::sleep;
use crate::ui::interpreter::UIReceiver;
use std::time::Duration;
use std::ops::Div;

mod interpreter;

const INTERPRETER_ADDRESS: &str = "127.0.0.1:38043";
const CYCLE_TIME: Duration = Duration::from_secs(1);

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
        let (sender, receiver) = channel::<Vec<String>>();

        let interpreter = Interpreter::new(sender);

        let (code_out, code_in) = channel::<String>();

        let sock = UdpSocket::bind(self.address).unwrap();
        println!("UI server listening on {}", self.address);

        thread::spawn(move || loop {
            match code_in.recv() {
                Ok(code) => {
                    interpreter.execute(code.to_string())
                },
                Err(e) => println!("Error receiving code: {}", e)
            }
        });

        thread::spawn(move || {
            let mut index = 0;
            let comand = command_out.clone();
            loop {
                match receiver.recv() {
                    Ok(pattern) => {
                        println!("New pattern! {}", pattern.len());
                        let cmd_out = comand.clone();
                        thread::spawn(move || {
                            loop {
                                if pattern.len() == 0 {
                                    sleep(CYCLE_TIME);
                                    continue;
                                }

                                if index == pattern.len() {
                                    index = 0;
                                }

                                cmd_out.send(Command::Instrument(pattern[index].clone()));

                                sleep(CYCLE_TIME.div(pattern.len() as u32));
                                index += 1;
                            }
                        });
                    },
                    Err(e) => println!("Error {}", e)
                }
            }
        });

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            let trimmed = message.trim();
                            println!("Received instruction:\n{}", trimmed);
                            match code_out.send(trimmed.to_string()) {
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

struct Interpreter {
    sender: Sender<Vec<String>>,
}

impl Interpreter {
    fn new(sender: Sender<Vec<String>>) -> Self {

        Interpreter { sender }
    }

    fn execute(&self, command: String) {
        let array = command
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        self.sender.send(array);
    }
}