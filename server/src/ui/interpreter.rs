use std::sync::mpsc::{Sender, Receiver};

pub struct Interpreter {
    code_in: Receiver<String>,
    pattern_out: Sender<Vec<String>>,
}

impl Interpreter {
    pub(crate) fn new(code_in: Receiver<String>, pattern_out: Sender<Vec<String>>) -> Self {
        Interpreter { code_in, pattern_out }
    }

    pub(crate) fn loop_forever(&self) {
        loop {
            match self.code_in.recv() {
                Ok(code) => {
                    let pattern = self.parse(code);
                    match self.pattern_out.send(pattern) {
                        Ok(_) => println!("Pattern sent correctly"),
                        Err(e) => println!("Error sending pattern: {}", e)
                    }
                },
                Err(e) => println!("Error receiving code: {}", e)
            }
        };
    }

    fn parse(&self, command: String) -> Vec<String> {
        command
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()
    }
}