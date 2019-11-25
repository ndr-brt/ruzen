use rosc::OscPacket;
use crate::synth::Command;
use std::error::Error;

pub(crate) fn message_to_command(packet: OscPacket) -> Result<Command, Box<dyn Error>> {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            let tokens: Vec<String> = msg.addr
                .split('/')
                .map(String::from)
                .collect();

            match tokens.last() {
                Some(name) => {
                    Result::Ok(Command::Instrument(name.to_string()))
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

#[cfg(test)]
mod tests {
    use super::message_to_command;
    use rosc::{OscMessage, OscPacket};
    use crate::synth::Command;

    #[test]
    fn play_kick() {
        let message = OscMessage {
            addr: "/instrument/kick".to_string(),
            args: Some(vec![])
        };

        let result = message_to_command(OscPacket::Message(message));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Instrument("kick".to_string()))
    }
}