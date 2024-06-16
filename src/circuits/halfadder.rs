use crate::gates::and::AndGate;
use crate::gates::gate::LogicGate;
use crate::gates::xor::XorGate;

/// A struct that represents a half adder.
///
/// A half adder is a basic circuit that performs the sum of two binary bits.
/// It has two input pins (`input_a` and `input_b`), and two output pins (`sum` and `carry`).
/// The `sum` pin contains the sum of the two input bits, and the `carry` pin contains the carry bit.
#[derive(Clone, Copy)]
pub struct HalfAdder {
    /// The first input bit.
    input_a: bool,
    /// The second input bit.
    input_b: bool,
}

impl HalfAdder {
    /// Creates a new instance of the half adder with the given input bits.
    ///
    /// # Arguments
    ///
    /// * `input_a` - The first input bit.
    /// * `input_b` - The second input bit.
    ///
    /// # Returns
    ///
    /// A new instance of the half adder.
    pub fn new(input_a: bool, input_b: bool) -> HalfAdder {
        HalfAdder { input_a, input_b }
    }

    /// Sets the values of the input bits.
    ///
    /// # Arguments
    ///
    /// * `input_a` - The new value for the first input bit.
    /// * `input_b` - The new value for the second input bit.
    pub fn set_inputs(&mut self, input_a: bool, input_b: bool) {
        self.input_a = input_a;
        self.input_b = input_b;
    }

    /// Computes the sum of the two input bits.
    ///
    /// # Returns
    ///
    /// The sum of the two input bits.
    pub fn sum(&self) -> bool {
        let xor_gate = XorGate::new([self.input_a, self.input_b]);
        xor_gate.output()
    }

    /// Computes the carry bit of the two input bits.
    ///
    /// # Returns
    ///
    /// The carry bit of the two input bits.
    pub fn carry(&self) -> bool {
        let and_gate = AndGate::new([self.input_a, self.input_b]);
        and_gate.output()
    }
}
