pub mod fulladder;
pub mod halfadder;
pub mod eightbitadder;

#[cfg(test)]
mod tests {
    use crate::circuits::eightbitadder::EightBitAdder;
    use crate::circuits::fulladder::FullAdder;
    use crate::circuits::halfadder::HalfAdder;

    #[test]
    fn half_adder() {
        let half_adder = HalfAdder::new(true, true);
        assert_eq!(half_adder.sum(), false);
        assert_eq!(half_adder.carry(), true);

        let half_adder = HalfAdder::new(false, true);
        assert_eq!(half_adder.sum(), true);
        assert_eq!(half_adder.carry(), false);

        let half_adder = HalfAdder::new(true, false);
        assert_eq!(half_adder.sum(), true);
        assert_eq!(half_adder.carry(), false);

        let half_adder = HalfAdder::new(false, false);
        assert_eq!(half_adder.sum(), false);
        assert_eq!(half_adder.carry(), false);
    }

    #[test]
    fn full_adder() {
        let full_adder = FullAdder::new(false, false, false);
        assert_eq!(full_adder.sum(), false);
        assert_eq!(full_adder.carry(), false);

        let full_adder = FullAdder::new(false, false, true);
        assert_eq!(full_adder.sum(), true);
        assert_eq!(full_adder.carry(), false);

        let full_adder = FullAdder::new(false, true, false);
        assert_eq!(full_adder.sum(), true);
        assert_eq!(full_adder.carry(), false);

        let full_adder = FullAdder::new(false, true, true);
        assert_eq!(full_adder.sum(), false);
        assert_eq!(full_adder.carry(), true);

        let full_adder = FullAdder::new(true, false, false);
        assert_eq!(full_adder.sum(), true);
        assert_eq!(full_adder.carry(), false);

        let full_adder = FullAdder::new(true, false, true);
        assert_eq!(full_adder.sum(), false);
        assert_eq!(full_adder.carry(), true);

        let full_adder = FullAdder::new(true, true, false);
        assert_eq!(full_adder.sum(), false);
        assert_eq!(full_adder.carry(), true);

        let full_adder = FullAdder::new(true, true, true);
        assert_eq!(full_adder.sum(), true);
        assert_eq!(full_adder.carry(), true);
    }

    #[test]
    fn eight_bit_adder() {
        let mut adder = EightBitAdder::new();

        let a = [true, false, true, false, true, false, true, false]; // 0b01010101 = 85
        let b = [false, true, false, true, false, true, false, true]; // 0b10101010 = 170

        let (sum, carry_out) = adder.add(a, b, false);

        assert_eq!(sum, [true, true, true, true, true, true, true, true]); // 0b11111111 = 255
        assert_eq!(carry_out, false);

        let (sum, carry_out) = adder.add(a, b, true);

        assert_eq!(sum, [false, false, false, false, false, false, false, false]); // 0b00000000 = 0
        assert_eq!(carry_out, true);
    }
}