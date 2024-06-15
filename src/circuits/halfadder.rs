use crate::gates::and::AndGate;
use crate::gates::gate::LogicGate;
use crate::gates::xor::XorGate;

pub struct HalfAdder {
    input_a: bool,
    input_b: bool,
}

impl HalfAdder {
    pub fn new(input_a: bool, input_b: bool) -> HalfAdder {
        HalfAdder { input_a, input_b }
    }

    pub fn set_inputs(&mut self, input_a: bool, input_b: bool) {
        self.input_a = input_a;
        self.input_b = input_b;
    }

    pub fn sum(&self) -> bool {
        let xor_gate = XorGate::new([self.input_a, self.input_b]);
        xor_gate.output()
    }

    pub fn carry(&self) -> bool {
        let and_gate = AndGate::new([self.input_a, self.input_b]);
        and_gate.output()
    }
}