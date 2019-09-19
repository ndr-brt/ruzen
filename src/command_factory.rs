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
                    let frequency = to_double(args[0].clone());
                    let phase = to_double(args[1].clone());

                    Result::Ok(Command::Play(wave(frequency, 0.), Wave::None, 1.))
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

fn to_double(arg: OscType) -> f64 {
    match arg {
        OscType::Double(value) => value,
        _ => 0.
    }
}

#[cfg(test)]
mod tests {
    use super::message_to_command;
    use rosc::{OscMessage, OscPacket, OscType};
    use crate::synth::Command;
    use crate::oscillator::Wave;

    #[test]
    fn play_sine_with_frequency_and_phase() {
        let message = OscMessage {
            addr: "/synth/sine".to_string(),
            args: Some(vec![OscType::Double(440.0), OscType::Double(1.)])
        };

        let result = message_to_command(OscPacket::Message(message));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Play(Wave::Sine(440., 0.), Wave::None, 1.), "aaaa")
    }
}