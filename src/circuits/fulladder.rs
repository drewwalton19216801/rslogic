//! Defines the FullAdder struct and its associated methods.

use crate::{
    gates::{gate::LogicGate, or::OrGate},
    circuits::halfadder::HalfAdder,
};

/// A struct that represents a full adder.
///
/// A full adder is a circuit that performs the sum of three binary bits.
/// It has three input pins (`input_a`, `input_b`, and `carry_in`), and two output pins (`sum` and `carry`).
/// The `sum` pin contains the sum of the three input bits, and the `carry` pin contains the carry bit.
#[derive(Clone, Copy)]
pub struct FullAdder {
    /// The first input bit.
    input_a: bool,
    /// The second input bit.
    input_b: bool,
    /// The carry bit.
    carry_in: bool,
}

impl FullAdder {
    /// Creates a new instance of the full adder with the given input bits.
    ///
    /// # Arguments
    ///
    /// * `input_a` - The first input bit.
    /// * `input_b` - The second input bit.
    /// * `carry_in` - The carry bit.
    ///
    /// # Returns
    ///
    /// A new instance of the full adder.
    pub fn new(input_a: bool, input_b: bool, carry_in: bool) -> FullAdder {
        FullAdder {
            input_a,
            input_b,
            carry_in,
        }
    }

    /// Sets the values of the input bits and the carry bit.
    ///
    /// # Arguments
    ///
    /// * `input_a` - The new value for the first input bit.
    /// * `input_b` - The new value for the second input bit.
    /// * `carry_in` - The new value for the carry bit.
    pub fn set_inputs(&mut self, input_a: bool, input_b: bool, carry_in: bool) {
        self.input_a = input_a;
        self.input_b = input_b;
        self.carry_in = carry_in;
    }

    /// Computes the sum of the three input bits.
    ///
    /// # Returns
    ///
    /// The sum of the three input bits.
    pub fn sum(&self) -> bool {
        // Create two half adders and compute their sums.
        let half_adder1 = HalfAdder::new(self.input_a, self.input_b);
        let sum1 = half_adder1.sum();
        let half_adder2 = HalfAdder::new(sum1, self.carry_in);
        half_adder2.sum()
    }

    /// Computes the carry bit of the three input bits.
    ///
    /// # Returns
    ///
    /// The carry bit of the three input bits.
    pub fn carry(&self) -> bool {
        // Create two half adders, compute their sums and carry bits, and create an OR gate.
        let half_adder1 = HalfAdder::new(self.input_a, self.input_b);
        let half_adder2 = HalfAdder::new(half_adder1.sum(), self.carry_in);
        let or_gate = OrGate::new([half_adder1.carry(), half_adder2.carry()]);
        or_gate.output()
    }
}

