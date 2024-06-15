pub mod hexinverter;
pub mod quadnand;

#[cfg(test)]
mod tests {
    use crate::chips::hexinverter::HexInverter;
    use crate::chips::quadnand::QuadNand;

    #[test]
    fn hex_inverter() {
        let inputs = [true, false, true, false, true, false];
        let mut hex_inverter = HexInverter::new(inputs);

        // Check initial outputs
        assert_eq!(hex_inverter.outputs(), [false, true, false, true, false, true]);

        // Change some inputs and check outputs again
        hex_inverter.set_input(0, false);
        hex_inverter.set_input(1, true);
        assert_eq!(hex_inverter.outputs(), [true, false, false, true, false, true]);

        // Check individual output
        assert_eq!(hex_inverter.output(0), true);
        assert_eq!(hex_inverter.output(1), false);
    }

    #[test]
    fn quad_nand() {
        let inputs = [
            (true, true),
            (false, true),
            (true, false),
            (false, false),
        ];
        let mut quad_nand = QuadNand::new(inputs);

        // Check initial outputs
        assert_eq!(quad_nand.outputs(), [false, true, true, true]);

        // Change some inputs and check outputs again
        quad_nand.set_inputs(0, false, false);
        quad_nand.set_inputs(1, true, true);
        assert_eq!(quad_nand.outputs(), [true, false, true, true]);

        // Check individual output
        assert_eq!(quad_nand.output(0), true);
        assert_eq!(quad_nand.output(1), false);
    }
}