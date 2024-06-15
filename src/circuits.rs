pub mod fulladder;
pub mod halfadder;

#[cfg(test)]
mod tests {
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
}