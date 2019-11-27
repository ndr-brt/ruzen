extern crate rosc;
extern crate rand;
#[macro_use]
extern crate gluon;
#[macro_use]
extern crate gluon_vm;

use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{UdpSocket};
use std::time::Duration;
use std::{f32, thread};
use rand::thread_rng;
use crate::rand::Rng;
use gluon::{ThreadExt, Thread};
use gluon::import::add_extern_module;
use gluon::vm;
use gluon::vm::api::primitive;
use gluon::vm::ExternModule;

type Hz = f64;

const HOST_ADDRESS: &str = "127.0.0.1:38122";
const SERVER_ADDRESS: &str = "127.0.0.1:38042";

fn play(name: &str) -> String {
    instrument(name).play();
    name.to_uppercase()
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

fn main() {
    let vm = gluon::new_vm();
    add_extern_module(&vm, "my_module", my_module);
    loop {
        let expr = r#"
            let module = import! "my_module"
            module.play "strange"
        "#;

        let result = vm
            .run_expr::<String>("example", expr)
            .ok();

        match result {
            Some((val, coso)) => println!("Result: {}", val),
            None => println!("No f**kn result")
        }

        sleep(300)
    }

}

//fn main() {
//    let pattern_duration = 1;
//
//    thread::spawn(move || loop { pattern("kick ~ kick ~", pattern_duration * 666) });
//    thread::spawn(move || loop { pattern("~ snare ~ snare", pattern_duration *1000) });
//    thread::spawn(move || loop { pattern("strange", pattern_duration *3000) });
//
//    loop {}
//}

fn pattern(pattern: &str, cycle_time: usize) {
    let tokens: Vec<&str> = pattern.split(" ").collect();
    let time_each: usize = cycle_time / tokens.len();
    tokens.iter().for_each(|token| {
        if *token != "~" {
            instrument(token).play();
        }
        sleep(time_each as u64);
    })
}

#[derive(Debug, Clone, Copy)]
struct Synth<'a> {
    name: &'a str,
    frequency: Hz,
    phase: f32,
    attack: f64,
    release: f64,
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

fn sleep(time: u64) {
    std::thread::sleep(Duration::from_millis(time));
}

fn rrand(from: f64, to: f64) -> f64 {
    thread_rng().gen_range::<f64, f64, f64>(from, to)
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