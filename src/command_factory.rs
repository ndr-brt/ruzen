use rosc::OscPacket;
use crate::synth::Command;
use std::error::Error;
use crate::oscillator::Wave;
use rosc::OscType;
use crate::clock::Hz;

pub(crate) fn message_to_command(packet: OscPacket) -> Result<Command, Box<dyn Error>> {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            let wave = match msg.addr.split('/').last() {
                Some("sine") => Wave::Sine,
                Some("saw") => Wave::Saw,
                Some(_) => {
                    println!("instrument not found, default is sine");
                    Wave::Sine
                }
                None => {
                    println!("instrument not found, default is sine");
                    Wave::Sine
                }
            };

            match msg.args {
                Some(args) => {
                    println!("OSC arguments: {:?}", args);
                    match args[0] {
                        OscType::Double(frequency) => {
                            Result::Ok(Command::Play(wave(frequency as Hz, 0.), Wave::None, 0.))
                        }
                        _ => {
                            Result::Err(Box::from("Not a valid frequency"))
                        }
                    }

                }
                None => {
                    Result::Err(Box::from("No arguments in message."))
                },
            }
        }
        OscPacket::Bundle(bundle) => {
            Result::Err(Box::from(format!("OSC Bundle: {:?}", bundle)))
        }
    }
}