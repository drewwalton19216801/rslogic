use crate::gates::gate::LogicGate;

/// A struct that represents a NOR gate.
///
/// This struct implements the `LogicGate` trait and provides functionality for
/// computing the output of the NOR gate given the inputs.
pub struct NorGate {
    /// The inputs of the NOR gate.
    inputs: [bool; 2],
}

impl LogicGate for NorGate {
    /// Computes the output of the NOR gate.
    ///
    /// # Returns
    ///
    /// The logical negation of the logical OR of the two inputs.
    fn output(&self) -> bool {
        // Compute the logical OR of the two inputs and take the logical negation.
        !(self.inputs[0] || self.inputs[1])
    }

    /// Sets the value of an input pin of the NOR gate.
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

impl NorGate {
    /// Creates a new instance of the NOR gate.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs of the NOR gate.
    ///
    /// # Returns
    ///
    /// A new instance of the NOR gate.
    pub fn new(inputs: [bool; 2]) -> NorGate {
        // Create a new instance of the NOR gate with the given inputs.
        NorGate { inputs }
    }
}
