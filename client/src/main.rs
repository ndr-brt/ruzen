extern crate rosc;
extern crate rand;
extern crate crossbeam;

use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{UdpSocket};
use std::{thread};
use std::sync::mpsc::{channel};
use interpreter::UIReceiver;
use std::thread::sleep;
use std::time::Duration;
use std::ops::Div;
use std::sync::Arc;
use crossbeam::channel::unbounded;
use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;

mod interpreter;

const HOST_ADDRESS: &str = "127.0.0.1:38122";
const SERVER_ADDRESS: &str = "127.0.0.1:38042";
const INTERPRETER_ADDRESS: &str = "127.0.0.1:38043";
const CYCLE_TIME: Duration = Duration::from_secs(1);

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

fn main() {
    let (sender, receiver) = unbounded::<Vec<String>>();

    let interpreter = Interpreter::new(sender);

    let (code_out, code_in) = channel::<String>();

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
        loop {
            match receiver.recv() {
                Ok(pattern) => {
                    println!("New pattern! {}", pattern.len());
                    thread::spawn(move || {
                        loop {
                            if pattern.len() == 0 {
                                sleep(CYCLE_TIME);
                                continue;
                            }

                            if index == pattern.len() {
                                index = 0;
                            }

                            let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                                addr: format!("/instrument/{}", pattern[index]),
                                args: Some(vec![]),
                            })).unwrap();

                            send_osc_message(msg_buf);

                            sleep(CYCLE_TIME.div(pattern.len() as u32));
                            index += 1;
                        }
                    });
                },
                Err(e) => println!("Error {}", e)
            }
        }
    });

    let interpreter = UIReceiver::new(INTERPRETER_ADDRESS);
    interpreter.listen(code_out);
}

fn send_osc_message(msg_buf: Vec<u8>) {
    match UdpSocket::bind(HOST_ADDRESS) {
        Ok(x) => {
            match x.send_to(&msg_buf, SERVER_ADDRESS) {
                Ok(_) => (),
                Err(err) => println!("Error in sending message to server {}", err)
            }
        },
        Err(_) => unimplemented!(),
    };
}
