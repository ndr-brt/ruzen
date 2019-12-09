use std::sync::mpsc::Sender;

pub struct Interpreter {
    sender: Sender<Vec<String>>,
}

impl Interpreter {
    pub(crate) fn new(sender: Sender<Vec<String>>) -> Self {

        Interpreter { sender }
    }

    pub(crate) fn execute(&self, command: String) {
        let array = command
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        self.sender.send(array);
    }
}