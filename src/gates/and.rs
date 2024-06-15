use crate::gates::gate::LogicGate;

/// A struct that represents an AND gate.
///
/// This struct implements the `LogicGate` trait and provides functionality for
/// computing the output of the AND gate given the inputs.
pub struct AndGate {
    /// The inputs of the AND gate.
    inputs: [bool; 2],
}

impl LogicGate for AndGate {
    /// Computes the output of the AND gate.
    ///
    /// # Returns
    ///
    /// The logical AND of the two inputs.
    fn output(&self) -> bool {
        // Compute the logical AND of the two inputs.
        self.inputs[0] && self.inputs[1]
    }

    /// Sets the value of an input pin of the AND gate.
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

impl AndGate {
    /// Creates a new instance of the AND gate.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs of the AND gate.
    ///
    /// # Returns
    ///
    /// A new instance of the AND gate.
    pub fn new(inputs: [bool; 2]) -> AndGate {
        // Create a new instance of the AND gate with the given inputs.
        AndGate { inputs }
    }
}
