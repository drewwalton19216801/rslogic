use crate::gates::gate::LogicGate;

pub struct OrGate {
    inputs: [bool; 2],
}

impl LogicGate for OrGate {
    fn output(&self) -> bool {
        self.inputs[0] || self.inputs[1]
    }

    fn set_input(&mut self, index: usize, value: bool) {
        self.inputs[index] = value;
    }
}

impl OrGate {
    pub fn new(inputs: [bool; 2]) -> OrGate {
        OrGate { inputs }
    }
}