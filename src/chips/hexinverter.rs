use crate::gates::gate::LogicGate;
use crate::gates::not::NotGate;

pub struct HexInverter {
    gates: [NotGate; 6],
}

impl HexInverter {
    pub fn new(inputs: [bool; 6]) -> HexInverter {
        HexInverter {
            gates: [
                NotGate::new(inputs[0]),
                NotGate::new(inputs[1]),
                NotGate::new(inputs[2]),
                NotGate::new(inputs[3]),
                NotGate::new(inputs[4]),
                NotGate::new(inputs[5]),
            ],
        }
    }

    pub fn set_input(&mut self, index: usize, value: bool) {
        if index < self.gates.len() {
            self.gates[index].set_input(0, value);
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

    pub fn outputs(&self) -> [bool; 6] {
        [
            self.output(0),
            self.output(1),
            self.output(2),
            self.output(3),
            self.output(4),
            self.output(5),
        ]
    }
}