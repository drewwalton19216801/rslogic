pub trait LogicGate {
    fn output(&self) -> bool;
    fn set_input(&mut self, index: usize, value: bool);
}