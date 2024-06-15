use crate::gates::{gate::LogicGate, nand::NandGate};

pub struct QuadNand {
    gates: [NandGate; 4],
}

impl QuadNand {
    pub fn new(inputs: [(bool, bool); 4]) -> QuadNand {
        QuadNand {
            gates: [
                NandGate::new([inputs[0].0, inputs[0].1]),
                NandGate::new([inputs[1].0, inputs[1].1]),
                NandGate::new([inputs[2].0, inputs[2].1]),
                NandGate::new([inputs[3].0, inputs[3].1]),
            ],
        }
    }

    pub fn set_inputs(&mut self, index: usize, input1: bool, input2: bool) {
        if index < self.gates.len() {
            self.gates[index].set_input(0, input1);
            self.gates[index].set_input(1, input2);
        } else {
            panic!("Index out of bounds");
        }
    }

    pub fn output(&self, index: usize) -> bool {
        if index < self.gates.len() {
            self.gates[index].output()
        } else {
            panic!("Index out of bounds");
        }
    }

    pub fn outputs(&self) -> [bool; 4] {
        [
            self.output(0),
            self.output(1),
            self.output(2),
            self.output(3),
        ]
    }
}