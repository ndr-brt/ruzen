extern crate rosc;
extern crate rand;
extern crate gluon;
#[macro_use]
extern crate gluon_vm;
extern crate crossbeam;

use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{UdpSocket};
use std::{thread};
use gluon::{ThreadExt, RootedThread};
use gluon::import::add_extern_module;
use gluon::vm::ExternModule;
use std::sync::mpsc::{channel};
use interpreter::GluonInterpreter;
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
    vm: RootedThread,
}

impl Interpreter {
    fn new(sender: Sender<Vec<String>>) -> Self {
/*
        let play = |command: String| {
            let array = command.split_whitespace().collect::<[&str]>();
            sender.send(array);
            command.to_uppercase()
        };
*/

        let vm = gluon::new_vm();
/*        let ah: fn(Interpreter, String) -> String = Interpreter::play;
        add_extern_module(&vm, "play", |thread| {
            ExternModule::new(thread, primitive!(1, ah))
        });

        match vm.load_file("ui.init") {
            Ok(_) => println!("Init script loaded"),
            Err(e) => println!("Init script not loaded: {}", e)
        }
*/
        Interpreter { sender, vm }
    }
/*
    fn play(self, command: String) -> String {
        let array = command.split_whitespace().collect::<Vec<&str>>();
        self.sender.send(array);
        command.to_uppercase()
    }
    */

    fn execute(&self, command: String) {
        let array = command
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        self.sender.send(array);
        /*
        let result = self.vm
            .run_expr::<String>("client", code.as_str())
            .ok();

        match result {
            Some((val, _arc_type)) => println!("Result: {}", val),
            None => println!("No result from gluon run_expr")
        }
        */
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

    let interpreter = GluonInterpreter::new(INTERPRETER_ADDRESS);
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
