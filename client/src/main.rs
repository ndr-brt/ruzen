extern crate rosc;
extern crate rand;
extern crate gluon;
#[macro_use]
extern crate gluon_vm;

use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{UdpSocket};
use std::{thread};
use gluon::{ThreadExt, Thread};
use gluon::import::add_extern_module;
use gluon::vm;
use gluon::vm::ExternModule;
use std::sync::mpsc::{channel};
use interpreter::GluonInterpreter;

mod interpreter;

const HOST_ADDRESS: &str = "127.0.0.1:38122";
const SERVER_ADDRESS: &str = "127.0.0.1:38042";
const INTERPRETER_ADDRESS: &str = "127.0.0.1:38043";

fn play_module(thread: &Thread) -> vm::Result<ExternModule> {
    ExternModule::new(
        thread,
        primitive!(1, play)
    )
}

fn my_module(thread: &Thread) -> vm::Result<ExternModule> {
    ExternModule::new(
        thread,
        record!{
            message => "Hello World!",
            play => primitive!(1, play),
        }
    )
}

fn play(name: &str) -> String {
    instrument(name).play();
    name.to_uppercase()
}

fn main() {
    let vm = gluon::new_vm();
    add_extern_module(&vm, "my_module", my_module);
    add_extern_module(&vm, "play", play_module);

    match vm.load_script("init.glu", "import! \"play\"") {
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
                   None => println!("No f**kn result")
               }
           },
           Err(e) => println!("Error receiving code: {}", e)
       }
    });

    let interpreter = GluonInterpreter::new(INTERPRETER_ADDRESS);
    interpreter.listen(code_out);
}

struct Instrument<'a> {
    name: &'a str
}
impl Instrument<'_> {
    pub fn new(name: &str) -> Instrument {
        Instrument {
            name
        }
    }

    pub fn play(&self) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: format!("/instrument/{}", self.name),
            args: Some(vec![]),
        })).unwrap();

        send_osc_message(msg_buf)
    }
}

fn instrument(name: &str) -> Instrument {
    Instrument::new(name)
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
