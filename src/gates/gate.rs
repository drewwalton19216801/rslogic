/// A trait that represents a logic gate.
///
/// This trait provides two methods: `output` and `set_input`.
///
/// `output` returns the output of the logic gate.
/// `set_input` sets the value of an input pin of the logic gate.
pub trait LogicGate {
    /// Returns the output of the logic gate.
    ///
    /// # Returns
    ///
    /// The output of the logic gate.
    fn output(&self) -> bool;

    /// Sets the value of an input pin of the logic gate.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the input pin.
    /// * `value` - The value to set the input pin to.
    fn set_input(&mut self, index: usize, value: bool);
}
