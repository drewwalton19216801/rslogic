use crate::gates::gate::LogicGate;

pub struct XnorGate {
    inputs: [bool; 2],
}

impl LogicGate for XnorGate {
    fn output(&self) -> bool {
        !(self.inputs[0] ^ self.inputs[1])
    }

    fn set_input(&mut self, index: usize, value: bool) {
        self.inputs[index] = value;
    }
}

impl XnorGate {
    pub fn new(inputs: [bool; 2]) -> XnorGate {
        XnorGate { inputs }
    }
}