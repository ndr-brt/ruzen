use crossbeam_channel::{Sender, unbounded, Receiver};
use rosc::{OscPacket, OscMessage, OscType, encoder};
use std::{str, thread};
use std::net::UdpSocket;
use crate::OSC_ADDRESS_CLIENT;
use rlua::{Lua, ExternalError};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use failure::_core::fmt::Error;
use failure::_core::ops::Deref;
use std::thread::sleep;
use std::ops::Div;
use std::time::Duration;

const CYCLE_TIME: Duration = Duration::from_secs(1);

pub struct Pattern {
    id: usize,
    definition: String,
}

pub struct Interpreter {
    osc_sink: Sender<OscPacket>,
    lua: Lua,
    patterns: HashMap<usize, Pattern>,
}

impl Interpreter {
    pub(crate) fn new(osc_address_out: &'static str) -> Interpreter {
        let lua = Lua::new();
        let mut patterns: HashMap<usize, String> = HashMap::new();
        let (osc_sink, osc_stream) = unbounded::<OscPacket>();
        let (pattern_sink, pattern_stream) = unbounded::<Pattern>();

        let pat_osc_sink = osc_sink.clone();
        thread::spawn(move || {
            let altro_clone = pat_osc_sink.clone();
            loop {
                match pattern_stream.recv() {
                    Ok(pattern) => {
                        let pieces = pattern.definition.split_whitespace()
                            .map(String::from)
                            .collect::<Vec<String>>();

                        let il_dio_can = altro_clone.clone();
                        thread::spawn(move || {
                            let mut index = 0;
                            loop {
                                if pieces.len() == 0 {
                                    sleep(CYCLE_TIME);
                                    continue;
                                }

                                if index == pieces.len() {
                                    index = 0;
                                }

                                il_dio_can.send(OscPacket::Message(OscMessage {
                                    addr: format!("/instrument/{}/anId", pieces[index]),
                                    args: vec![],
                                }));

                                sleep(CYCLE_TIME.div(pieces.len() as u32));
                                index += 1;
                            }
                        });
                    }
                    Err(e) => {}
                }
            }
        });

        let _: Result<(), Error> = lua.context(|lua_ctx| {
            match read_file("src/ui/ui.lua".to_string()) {
                Ok(script) => { lua_ctx.load(&script).exec(); }
                Err(e) => println!("Error reading script: {}", e.to_string())
            };

            let globals = lua_ctx.globals();

            let inst_osc_sink = osc_sink.clone();
            let fun = lua_ctx.create_function(move |_, (id, name, params): (String, String, HashMap::<String, String>)| {
                println!("Instrument: {}", name);
                println!("Parames: {:?}", params);
                let mut osc_params = Vec::new();
                for x in params {
                    osc_params.push(OscType::String(x.0));
                    osc_params.push(OscType::String(x.1));
                }
                inst_osc_sink.send(OscPacket::Message(OscMessage {
                    addr: format!("/instrument/{}/{}", name, id),
                    args: osc_params,
                }));

                Ok(id)
            });
            globals.set("inst", fun.unwrap());

            let pattern_code_sink = osc_sink.clone();
            globals.set("p", lua_ctx.create_function(move |_, (id, definition): (usize, String)| {
                pattern_sink.send(Pattern { id, definition });

                Ok(())
            }).unwrap());

            let hush_code_sink = osc_sink.clone();
            globals.set("hush", lua_ctx.create_function(move |_, ()| {
                hush_code_sink.send(OscPacket::Message(OscMessage {
                    addr: format!("/hush"),
                    args: vec![],
                }));

                Ok(())
            }).unwrap());

            Ok(())
        });


        thread::spawn(move || {
            let socket = UdpSocket::bind(OSC_ADDRESS_CLIENT).unwrap();

            loop {
                match osc_stream.recv() {
                    Ok(osc) => {
                        match socket.send_to(encoder::encode(&osc).unwrap().as_slice(), osc_address_out) {
                            Ok(size) => println!("Sent {} osc bytes to server", size),
                            Err(e) => println!("Error sending osc message to server: {}", e.to_string())
                        }
                    }
                    Err(e) => println!("Some error receiving osc message to send: {}", e.to_string())
                }
            }
        });

        Interpreter { osc_sink, lua, patterns: HashMap::new() }
    }

    pub(crate) fn sender(&self) -> Sender<OscPacket> {
        self.osc_sink.clone()
    }

    pub(crate) fn init(&'static mut self) {}

    pub(crate) fn run(&self, code: String) {
        self.lua.context(|context| {
            match context
                .load(&code)
                .set_name("example code")
                .unwrap()
                .exec() {
                Ok(_) => {}
                Err(e) => println!("Error evaluating code: {}", e.to_string())
            }
        });
    }
}

fn read_file(path: String) -> Result<String, std::io::Error> {
    let mut script_file = File::open(path)?;
    let mut script = String::new();

    match script_file.read_to_string(&mut script) {
        Ok(_size) => Ok(script),
        Err(e) => Err(e)
    }
}