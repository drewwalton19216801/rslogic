use crate::gates::gate::LogicGate;

/// A struct that represents a NAND gate.
///
/// This struct implements the `LogicGate` trait and provides functionality for
/// computing the output of the NAND gate given the inputs.
pub struct NandGate {
    /// The inputs of the NAND gate.
    inputs: [bool; 2],
}

impl LogicGate for NandGate {
    /// Computes the output of the NAND gate.
    ///
    /// # Returns
    ///
    /// The logical negation of the logical AND of the two inputs.
    fn output(&self) -> bool {
        // Compute the logical AND of the two inputs and take the logical negation.
        !(self.inputs[0] && self.inputs[1])
    }

    /// Sets the value of an input pin of the NAND gate.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the input pin.
    /// * `value` - The value to set the input pin to.
    fn set_input(&mut self, index: usize, value: bool) {
        // Set the value of the input pin at the given index.
        self.inputs[index] = value;
    }
}

impl NandGate {
    /// Creates a new instance of the NAND gate.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs of the NAND gate.
    ///
    /// # Returns
    ///
    /// A new instance of the NAND gate.
    pub fn new(inputs: [bool; 2]) -> NandGate {
        // Create a new instance of the NAND gate with the given inputs.
        NandGate { inputs }
    }
}
