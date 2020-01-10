extern crate rlua;
extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::{str, thread};
use std::sync::mpsc::{Sender, channel};
use crate::synth::Command;
use std::time::Duration;
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use crate::{OSC_ADDRESS_CLIENT, OSC_ADDRESS_SERVER};
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

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

    fn inst(&mut self, name: String) {
        println!("Instrument: {}", name);
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/instrument/".to_owned() + &name, // TODO: use string format
            args: vec![],
        })).unwrap();

        self.sender.send(msg_buf);
    }

    fn sender(&self) -> Sender<Vec<u8>> {
        self.sender.clone()
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

        let lua = Lua::new();
        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            let socket = UdpSocket::bind(OSC_ADDRESS_CLIENT).unwrap();
            let interpreter = Interpreter::new(self.osc_address_server);

            let sender_clone = interpreter.sender();

            match lua_ctx.create_function(move |_, (name): (String)| {
                println!("Instrument: {}", name);
                let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                    addr: "/instrument/".to_owned() + &name, // TODO: use string format
                    args: vec![],
                })).unwrap();

                sender_clone.send(msg_buf);

                Ok(())
            }) {
                Ok(function) => { globals.set("play", function); },
                Err(e) => println!("Error loading function play {}", e.to_string())
            }
        });

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match code_sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            println!("Received instruction:\n{}", message);
                            lua.context(|context| {
                                match context
                                    .load(message)
                                    .set_name("example code")
                                    .unwrap()
                                    .exec() {
                                    Ok(_) => {},
                                    Err(e) => println!("Error evaluating code: {}", e.to_string())
                                }
                            });
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