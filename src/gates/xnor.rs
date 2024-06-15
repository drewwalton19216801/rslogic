use crate::gates::gate::LogicGate;

/// A struct that represents an XNOR gate.
/// 
/// This struct implements the `LogicGate` trait and provides functionality for
/// computing the output of the XNOR gate given the inputs.
pub struct XnorGate {
    /// The inputs of the XNOR gate.
    inputs: [bool; 2],
}

impl LogicGate for XnorGate {
    /// Computes the output of the XNOR gate.
    ///
    /// # Returns
    ///
    /// The logical XNOR of the two inputs.
    fn output(&self) -> bool {
        // Compute the logical XNOR of the two inputs.
        !(self.inputs[0] ^ self.inputs[1])
    }

    /// Sets the value of an input pin of the XNOR gate.
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

impl XnorGate {
    /// Creates a new instance of the XNOR gate.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs of the XNOR gate.
    ///
    /// # Returns
    ///
    /// A new instance of the XNOR gate.
    pub fn new(inputs: [bool; 2]) -> XnorGate {
        // Create a new instance of the XNOR gate with the given inputs.
        XnorGate { inputs }
    }
}