use crate::circuits::fulladder::FullAdder;

#[derive(Clone, Copy)]
pub struct EightBitAdder {
    full_adders: [FullAdder; 8],
    carry_out: bool,
}

impl EightBitAdder {
    pub fn new() -> EightBitAdder {
        let initial_adder = FullAdder::new(false, false, false);
        EightBitAdder {
            full_adders: [initial_adder; 8],
            carry_out: false,
        }
    }

    pub fn add(&mut self, a: [bool; 8], b: [bool; 8], carry_in: bool) -> ([bool; 8], bool) {
        let mut carry = carry_in;
        let mut sum = [false; 8];

        for i in 0..8 {
            self.full_adders[i] = FullAdder::new(a[i], b[i], carry);
            sum[i] = self.full_adders[i].sum();
            carry = self.full_adders[i].carry();
        }

        self.carry_out = carry;
        (sum, self.carry_out)
    }
}