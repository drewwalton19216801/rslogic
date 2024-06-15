pub(crate) mod hexinverter;

#[cfg(test)]
mod tests {
    use crate::chips::hexinverter::HexInverter;

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
}