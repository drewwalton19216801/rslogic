use crate::gates::gate::LogicGate;

/// A struct that represents a NOT gate.
///
/// This struct implements the `LogicGate` trait and provides functionality for
/// computing the output of the NOT gate given the input.
pub struct NotGate {
    /// The input of the NOT gate.
    input: bool,
}

impl LogicGate for NotGate {
    /// Computes the output of the NOT gate.
    ///
    /// # Returns
    ///
    /// The logical negation of the input.
    fn output(&self) -> bool {
        // Compute the logical negation of the input.
        !self.input
    }

    /// Sets the value of the input pin of the NOT gate.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the input pin. It is ignored because the NOT gate has only one input.
    /// * `value` - The value to set the input pin to.
    fn set_input(&mut self, _index: usize, value: bool) {
        // Set the value of the input pin.
        self.input = value;
    }
}

impl NotGate {
    /// Creates a new instance of the NOT gate.
    ///
    /// # Arguments
    ///
    /// * `input` - The input of the NOT gate.
    ///
    /// # Returns
    ///
    /// A new instance of the NOT gate.
    pub fn new(input: bool) -> NotGate {
        // Create a new instance of the NOT gate with the given input.
        NotGate { input }
    }
}
