use crate::gates::gate::LogicGate;

/// A struct that represents an XOR gate.
///
/// This struct implements the `LogicGate` trait and provides functionality for
/// computing the output of the XOR gate given the inputs.
pub struct XorGate {
    /// The inputs of the XOR gate.
    inputs: [bool; 2],
}

impl LogicGate for XorGate {
    /// Computes the output of the XOR gate.
    ///
    /// # Returns
    ///
    /// The logical XOR of the two inputs.
    fn output(&self) -> bool {
        // Compute the logical XOR of the two inputs.
        self.inputs[0] ^ self.inputs[1]
    }

    /// Sets the value of an input pin of the XOR gate.
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

impl XorGate {
    /// Creates a new instance of the XOR gate.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs of the XOR gate.
    ///
    /// # Returns
    ///
    /// A new instance of the XOR gate.
    pub fn new(inputs: [bool; 2]) -> XorGate {
        // Create a new instance of the XOR gate with the given inputs.
        XorGate { inputs }
    }
}
