use crate::{gates::{gate::LogicGate, or::OrGate}, circuits::halfadder::HalfAdder};

pub struct FullAdder {
    input_a: bool,
    input_b: bool,
    carry_in: bool,
}

impl FullAdder {
    pub fn new(input_a: bool, input_b: bool, carry_in: bool) -> FullAdder {
        FullAdder {
            input_a,
            input_b,
            carry_in,
        }
    }

    pub fn set_inputs(&mut self, input_a: bool, input_b: bool, carry_in: bool) {
        self.input_a = input_a;
        self.input_b = input_b;
        self.carry_in = carry_in;
    }

    pub fn sum(&self) -> bool {
        let half_adder1 = HalfAdder::new(self.input_a, self.input_b);
        let sum1 = half_adder1.sum();
        let half_adder2 = HalfAdder::new(sum1, self.carry_in);
        half_adder2.sum()
    }

    pub fn carry(&self) -> bool {
        let half_adder1 = HalfAdder::new(self.input_a, self.input_b);
        let half_adder2 = HalfAdder::new(half_adder1.sum(), self.carry_in);
        let or_gate = OrGate::new([half_adder1.carry(), half_adder2.carry()]);
        or_gate.output()
    }
}
