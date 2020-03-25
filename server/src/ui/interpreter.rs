use crossbeam_channel::{Sender, unbounded, Receiver};
use rosc::{OscPacket, OscMessage, OscType, encoder};
use std::{str, thread};
use std::net::UdpSocket;
use crate::OSC_ADDRESS_CLIENT;
use rlua::{Lua};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use failure::_core::fmt::Error;
use std::thread::sleep;
use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Pattern {
    id: usize,
    definition: String,
}

#[derive(Copy, Clone)]
pub struct Cycle {
    number: u16,
    duration: Duration,
}

pub struct Interpreter {
    osc_sink: Sender<OscPacket>,
    lua: Lua,
    patterns: Arc<Mutex<HashMap<usize, Pattern>>>,
}

impl Interpreter {
    pub(crate) fn new(osc_address_out: &'static str) -> Interpreter {
        let lua = Lua::new();
        let patterns: HashMap<usize, Pattern> = HashMap::new();
        let patterns_arc = Arc::new(Mutex::new(patterns));

        let (osc_sink, osc_stream) = unbounded::<OscPacket>();
        let (pattern_sink, pattern_stream) = unbounded::<Pattern>();
        let (timer_sink, timer_stream) = unbounded::<Cycle>();
        thread::spawn(move || {
            let mut number = 0;
            loop {
                let duration = Duration::from_secs(1);
                sleep(duration);
                timer_sink.send(Cycle { number, duration });
                number += 1;
            }
        });

        {
            let patterns_arc = patterns_arc.clone();
            let osc_sink = osc_sink.clone();
            thread::spawn(move || Interpreter::handle_patterns(timer_stream, patterns_arc, osc_sink));
        }

        {
            let patterns_arc = patterns_arc.clone();
            thread::spawn(move || Interpreter::listen_pattern_change(pattern_stream, patterns_arc));
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

        Interpreter { osc_sink, lua, patterns: patterns_arc }
    }

    fn handle_patterns(timer_stream: Receiver<Cycle>, patterns_arc: Arc<Mutex<HashMap<usize, Pattern>>>, osc_sink: Sender<OscPacket>) {
        loop {
            match timer_stream.recv() {
                Ok(cycle) => {
                    let patterns = patterns_arc.lock().unwrap();
                    for (id, pattern) in patterns.iter() {
                        let definition = &pattern.definition;
                        let pieces = definition.split_whitespace()
                            .map(String::from)
                            .collect::<Vec<String>>();

                        let pattern_osc_sink = osc_sink.clone();
                        let io_dio = id.clone();
                        thread::spawn(move || Interpreter::send(cycle, pieces, pattern_osc_sink, io_dio));
                    }
                },
                Err(e) => { println!("Error receiving time {}", e.to_string()); }
            }
        }
    }

    fn send(cycle: Cycle, pieces: Vec<String>, pattern_osc_sink: Sender<OscPacket>, id: usize) -> () {
        let mut index = 0;
        while index < pieces.len() {
            pattern_osc_sink.send(OscPacket::Message(OscMessage {
                addr: format!("/instrument/{}/{}-{}", pieces[index], id, index),
                args: vec![],
            }));

            index += 1;
            sleep(cycle.duration.div(pieces.len() as u32));
        }
    }

    fn listen_pattern_change(pattern_stream: Receiver<Pattern>, patterns: Arc<Mutex<HashMap<usize, Pattern>>>) {
        loop {
            match pattern_stream.recv() {
                Ok(pattern) => {
                    patterns.lock().unwrap().insert(pattern.id, pattern);
                }
                Err(_e) => {}
            };
        }
    }

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