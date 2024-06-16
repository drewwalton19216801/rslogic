use crate::gates::{gate::LogicGate, nor::NorGate};

pub struct QuadNor {
    gates: [NorGate; 4],
}

impl QuadNor {
    pub fn new(inputs: [[bool; 2]; 4]) -> Self {
        QuadNor {
            gates: [
                NorGate::new(inputs[0]),
                NorGate::new(inputs[1]),
                NorGate::new(inputs[2]),
                NorGate::new(inputs[3]),
            ]
        }
    }

    pub fn set_inputs(&mut self, index: usize, inputs: [bool; 2]) {
        if index < self.gates.len() {
            self.gates[index].set_input(0, inputs[0]);
            self.gates[index].set_input(1, inputs[1]);
        } else {
            panic!("Index out of bounds");
        }
    }

    pub fn outputs(&self) -> [bool; 4] {
        [
            self.gates[0].output(),
            self.gates[1].output(),
            self.gates[2].output(),
            self.gates[3].output(),
        ]
    }
}