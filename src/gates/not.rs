use crate::gates::gate::LogicGate;

pub struct NotGate {
    input: bool,
}

impl LogicGate for NotGate {
    fn output(&self) -> bool {
        !self.input
    }

    fn set_input(&mut self, _index: usize, value: bool) {
        self.input = value;
    }
}

impl NotGate {
    pub fn new(input: bool) -> NotGate {
        NotGate { input }
    }
}