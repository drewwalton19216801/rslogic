use crate::gates::gate::LogicGate;

pub struct XorGate {
    inputs: [bool; 2],
}

impl LogicGate for XorGate {
    fn output(&self) -> bool {
        self.inputs[0] ^ self.inputs[1]
    }

    fn set_input(&mut self, index: usize, value: bool) {
        self.inputs[index] = value;
    }
}

impl XorGate {
    pub fn new(inputs: [bool; 2]) -> XorGate {
        XorGate { inputs }
    }
}