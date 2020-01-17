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
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
        let patterns: HashMap<usize, String> = HashMap::new();
        let mutex_patterns: Mutex<HashMap<usize, String>> = Mutex::new(patterns);
        let arc = Arc::new(mutex_patterns);
        let (osc_sink, osc_stream) = unbounded::<OscPacket>();
        let (pattern_sink, pattern_stream) = unbounded::<Pattern>();
        let (timer_sink, timer_stream) = unbounded::<String>();
        thread::spawn(move || {
            loop {
                sleep(Duration::from_secs(1));
                timer_sink.send("cacca".to_string());
            }
        });

        let barc = arc.clone();
        let timer_sink = osc_sink.clone();
        thread::spawn(move || {
            loop {
                match timer_stream.recv() {
                    Ok(time) => {
                        println!("ah ostia");
                        let patterns = barc.lock().unwrap();
                        println!("Ci sono dei pattern? {}", patterns.len());
                        for (id, definition) in patterns.iter() {
                            let pieces = definition.split_whitespace()
                                .map(String::from)
                                .collect::<Vec<String>>();

                            let pat_sin = timer_sink.clone();
                            thread::spawn(move || {
                                let mut index = 0;
                                while index < pieces.len() {
                                    pat_sin.send(OscPacket::Message(OscMessage {
                                        addr: format!("/instrument/{}/{}", pieces[index], index),
                                        args: vec![],
                                    }));

                                    index += 1;
                                    sleep(Duration::from_secs_f64((1. / (pieces.len() as f64)) as f64));
                                }
                            });
                        }
                    },
                    Err(e) => { println!("Error receiving time {}", e.to_string()); }
                }
            }
        });

        {
            let arc = arc.clone();
            thread::spawn(move || {
                loop {
                    match pattern_stream.recv() {
                        Ok(pattern) => {
                            let mut patterns = arc.lock().unwrap();
                            patterns.insert(pattern.id, pattern.definition);
                        }
                        Err(e) => {}
                    }
                }
            });
        }


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