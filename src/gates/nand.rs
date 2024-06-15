use crate::gates::gate::LogicGate;

pub struct NandGate {
    inputs: [bool; 2],
}

impl LogicGate for NandGate {
    fn output(&self) -> bool {
        !(self.inputs[0] && self.inputs[1])
    }

    fn set_input(&mut self, index: usize, value: bool) {
        self.inputs[index] = value;
    }
}

impl NandGate {
    pub fn new(inputs: [bool; 2]) -> NandGate {
        NandGate { inputs }
    }
}