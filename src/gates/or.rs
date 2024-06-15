use crate::gates::gate::LogicGate;

/// A struct that represents an OR gate.
///
/// This struct implements the `LogicGate` trait and provides functionality for
/// computing the output of the OR gate given the inputs.
pub struct OrGate {
    /// The inputs of the OR gate.
    inputs: [bool; 2],
}

impl LogicGate for OrGate {
    /// Computes the output of the OR gate.
    ///
    /// # Returns
    ///
    /// The logical OR of the two inputs.
    fn output(&self) -> bool {
        // Compute the logical OR of the two inputs.
        self.inputs[0] || self.inputs[1]
    }

    /// Sets the value of an input pin of the OR gate.
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

impl OrGate {
    /// Creates a new instance of the OR gate.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs of the OR gate.
    ///
    /// # Returns
    ///
    /// A new instance of the OR gate.
    pub fn new(inputs: [bool; 2]) -> OrGate {
        // Create a new instance of the OR gate with the given inputs.
        OrGate { inputs }
    }
}
