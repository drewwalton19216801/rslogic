use crate::gates::gate::LogicGate;
use crate::gates::not::NotGate;

/// A struct that represents a hex inverter.
///
/// A hex inverter is a circuit that performs the logical negation of six binary bits.
/// It has six input pins and six output pins. Each output pin contains the logical
/// negation of the corresponding input pin.
pub struct HexInverter {
    /// The logical inverter gates.
    gates: [NotGate; 6],
}

impl HexInverter {
    /// Creates a new instance of the hex inverter.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs of the hex inverter.
    ///
    /// # Returns
    ///
    /// A new instance of the hex inverter.
    pub fn new(inputs: [bool; 6]) -> HexInverter {
        HexInverter {
            gates: [
                NotGate::new(inputs[0]),
                NotGate::new(inputs[1]),
                NotGate::new(inputs[2]),
                NotGate::new(inputs[3]),
                NotGate::new(inputs[4]),
                NotGate::new(inputs[5]),
            ],
        }
    }

    /// Sets the value of an input pin of the hex inverter.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the input pin.
    /// * `value` - The value to set the input pin to.
    pub fn set_input(&mut self, index: usize, value: bool) {
        // Check if the index is within bounds
        if index < self.gates.len() {
            // Set the value of the input pin
            self.gates[index].set_input(0, value);
        } else {
            // Panic if the index is out of bounds
            panic!("Index out of bounds");
        }
    }

    /// Returns the output of the hex inverter for a specific output pin.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the output pin.
    ///
    /// # Returns
    ///
    /// The logical negation of the corresponding input pin.
    pub fn output(&self, index: usize) -> bool {
        // Check if the index is within bounds
        if index < self.gates.len() {
            // Return the output of the gate
            self.gates[index].output()
        } else {
            // Panic if the index is out of bounds
            panic!("Index out of bounds");
        }
    }

    /// Returns the outputs of all output pins of the hex inverter.
    ///
    /// # Returns
    ///
    /// An array containing the logical negations of all input pins.
    pub fn outputs(&self) -> [bool; 6] {
        [
            self.output(0),
            self.output(1),
            self.output(2),
            self.output(3),
            self.output(4),
            self.output(5),
        ]
    }
}
