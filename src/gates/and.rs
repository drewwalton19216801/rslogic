use crate::gates::gate::LogicGate;

pub struct AndGate {
    inputs: [bool; 2],
}

impl LogicGate for AndGate {
    fn output(&self) -> bool {
        self.inputs[0] && self.inputs[1]
    }

    fn set_input(&mut self, index: usize, value: bool) {
        self.inputs[index] = value;
    }
}

impl AndGate {
    pub fn new(inputs: [bool; 2]) -> AndGate {
        AndGate {
            inputs,
        }
    }
}