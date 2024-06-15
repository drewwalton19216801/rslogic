use crate::gates::gate::LogicGate;

pub struct NorGate {
    inputs: [bool; 2],
}

impl LogicGate for NorGate {
    fn output(&self) -> bool {
        !(self.inputs[0] || self.inputs[1])
    }

    fn set_input(&mut self, index: usize, value: bool) {
        self.inputs[index] = value;
    }
}

impl NorGate {
    pub fn new(inputs: [bool; 2]) -> NorGate {
        NorGate { inputs }
    }
}