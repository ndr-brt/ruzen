use rosc::OscPacket;
use crate::synth::Command;
use std::error::Error;
use crate::oscillator::Wave;
use rosc::OscType;
use crate::clock::Hz;
use crate::instrument::Instruments;
use crate::instrument::Instruments::{Kick, Snare};

pub(crate) fn message_to_command(packet: OscPacket) -> Result<Command, Box<dyn Error>> {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            let address_tokens = msg.addr.split('/');

            match address_tokens.last() {
                Some("kick") => {
                    Result::Ok(Command::Instrument(Kick))
                },
                Some("snare") => {
                    Result::Ok(Command::Instrument(Snare))
                }
                Some(_) => {
                    Result::Err(Box::from("instrument not found"))
                }
                None => {
                    Result::Err(Box::from("instrument not found"))
                }
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
    use crate::instrument::Instruments;

    #[test]
    fn play_kick() {
        let message = OscMessage {
            addr: "/instrument/kick".to_string(),
            args: Some(vec![])
        };

        let result = message_to_command(OscPacket::Message(message));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Instrument(Instruments::Kick))
    }
}