pub(crate) mod and;
pub(crate) mod gate;
pub(crate) mod or;
pub(crate) mod not;
pub(crate) mod nand;
pub(crate) mod nor;
pub(crate) mod xor;
pub(crate) mod xnor;

#[cfg(test)]
mod tests {
    use crate::gates::and::AndGate;
    use crate::gates::gate::LogicGate;
    use crate::gates::nand::NandGate;
    use crate::gates::nor::NorGate;
    use crate::gates::not::NotGate;
    use crate::gates::or::OrGate;
    use crate::gates::xnor::XnorGate;
    use crate::gates::xor::XorGate;

    #[test]
    fn and_gate() {
        let mut and_gate = AndGate::new([true, true]);
        assert_eq!(and_gate.output(), true);
        and_gate.set_input(0, false);
        assert_eq!(and_gate.output(), false);
    }
    #[test]
    fn nand_gate() {
        let mut nand_gate = NandGate::new([true, true]);
        assert_eq!(nand_gate.output(), false);
        nand_gate.set_input(0, false);
        assert_eq!(nand_gate.output(), true);
    }

    #[test]
    fn nor_gate() {
        let mut nor_gate = NorGate::new([true, true]);
        assert_eq!(nor_gate.output(), false);
        nor_gate.set_input(0, false);
        assert_eq!(nor_gate.output(), false);
        nor_gate.set_input(1, false);
        assert_eq!(nor_gate.output(), true);
    }

    #[test]
    fn not_gate() {
        let mut not_gate = NotGate::new(true);
        assert_eq!(not_gate.output(), false);
        not_gate.set_input(0, false); // Not gates only have one input
        assert_eq!(not_gate.output(), true);
    }

    #[test]
    fn or_gate() {
        let mut or_gate = OrGate::new([true, true]);
        assert_eq!(or_gate.output(), true);
        or_gate.set_input(0, false);
        assert_eq!(or_gate.output(), true);
    }

    #[test]
    fn xnor_gate() {
        let mut xnor_gate = XnorGate::new([true, true]);
        assert_eq!(xnor_gate.output(), true);
        xnor_gate.set_input(0, false);
        assert_eq!(xnor_gate.output(), false);
    }

    #[test]
    fn xor_gate() {
        let mut xor_gate = XorGate::new([true, true]);
        assert_eq!(xor_gate.output(), false);
        xor_gate.set_input(0, false);
        assert_eq!(xor_gate.output(), true);
    }
}