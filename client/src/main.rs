extern crate rosc;
extern crate rand;
extern crate gluon;
#[macro_use]
extern crate gluon_vm;

use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{UdpSocket};
use std::{thread};
use gluon::{ThreadExt};
use gluon::import::add_extern_module;
use gluon::vm::ExternModule;
use std::sync::mpsc::{channel};
use interpreter::GluonInterpreter;
use std::thread::sleep;
use std::time::Duration;

mod interpreter;

const HOST_ADDRESS: &str = "127.0.0.1:38122";
const SERVER_ADDRESS: &str = "127.0.0.1:38042";
const INTERPRETER_ADDRESS: &str = "127.0.0.1:38043";

fn play(command: String) -> String {
    command
        .split_whitespace()
        .for_each(|name| {
            if name == "~" {
                sleep(Duration::from_millis(300))
            } else {
                let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                    addr: format!("/instrument/{}", name),
                    args: Some(vec![]),
                })).unwrap();

                send_osc_message(msg_buf);
            }
    });

    command.to_uppercase()
}

fn main() {
    let vm = gluon::new_vm();
    add_extern_module(&vm, "play", |thread|
        ExternModule::new(thread, primitive!(1, play))
    );

    match vm.load_file("ui.init") {
        Ok(_) => println!("Init script loaded"),
        Err(e) => println!("Init script not loaded: {}", e)
    }

    let (code_out, code_in) = channel::<String>();

    thread::spawn(move || loop {
       match code_in.recv() {
           Ok(code) => {
               let result = vm
                   .run_expr::<String>("client", code.as_str())
                   .ok();

               match result {
                   Some((val, _arc_type)) => println!("Result: {}", val),
                   None => println!("No result from gluon run_expr")
               }
           },
           Err(e) => println!("Error receiving code: {}", e)
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
