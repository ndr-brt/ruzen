extern crate rlua;
extern crate rosc;

use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::{str, thread};
use rosc::{OscMessage, OscPacket, OscType};
use crate::{OSC_ADDRESS_CLIENT};
use rlua::{Lua, Result};
use std::fs::File;
use std::io::Read;
use self::rlua::ExternalError;
use crate::ui::interpreter::Interpreter;
use std::collections::HashMap;
use std::sync::mpsc::channel;

mod interpreter;

pub struct UIServer {
    address: SocketAddrV4,
    osc_address_server: &'static str,
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

    pub fn listen(&self) -> Result<()> {
        let interpreter = Interpreter::new(self.osc_address_server);
        let lua = init_lua(interpreter).unwrap();

        let code_sock = UdpSocket::bind(self.address).unwrap();
        println!("UI server listening on {}", self.address);
        let (code_sink, code_stream) = channel::<String>();

        thread::spawn(move || {
            loop {
                match code_stream.recv() {
                    Ok(code) => {
                        lua.context(|context| {
                            match context
                                .load(&code)
                                .set_name("example code")
                                .unwrap()
                                .exec() {
                                Ok(_) => {},
                                Err(e) => println!("Error evaluating code: {}", e.to_string())
                            }
                        });
                    },
                    Err(_e) => {}
                }
            }
        });

        let mut buf = [0u8; rosc::decoder::MTU];
        loop {
            match code_sock.recv_from(&mut buf) {
                Ok((size, _address)) => {
                    match str::from_utf8(&buf[..size]) {
                        Ok(message) => {
                            println!("Received instruction:\n{}", message);
                            code_sink.send(message.to_string());
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

fn init_lua(interpreter: Interpreter) -> Result<Lua> {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        let script: String = read_file("src/ui/ui.lua".to_string())?;
        lua_ctx.load(&script).exec()?;

        let globals = lua_ctx.globals();

        let sender_clone2 = interpreter.sender();
        let fun = lua_ctx.create_function(move |_, (id, name, params): (String, String, HashMap::<String, String>)| {
            println!("Instrument: {}", name);
            println!("Parames: {:?}", params);
            let mut osc_params = Vec::new();
            for x in params {
                osc_params.push(OscType::String(x.0));
                osc_params.push(OscType::String(x.1));
            }
            sender_clone2.send(OscPacket::Message(OscMessage {
                addr: format!("/instrument/{}/{}", name, id),
                args: osc_params,
            }));

            Ok(id)
        })?;
        globals.set("inst", fun)?;

        let sender_clone3 = interpreter.sender();
        globals.set("hush", lua_ctx.create_function(move |_, ()| {
            sender_clone3.send(OscPacket::Message(OscMessage {
                addr: format!("/hush"),
                args: vec![]
            }));

            Ok(())
        })?)?;

        Ok(())
    })?;

    Ok(lua)
}

fn read_file(path: String) -> Result<String> {
    let mut script_file = File::open(path).expect("could not open script");
    let mut script = String::new();

    match script_file.read_to_string(&mut script) {
        Ok(_size) => Ok(script),
        Err(e) => Err(e.to_lua_err())
    }
}