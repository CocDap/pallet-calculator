use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn it_works_for_calculator() {
    new_test_ext().execute_with(|| {
        // Add
        assert_ok!(Calculator::do_add(RuntimeOrigin::signed(1), 42));
        assert_eq!(Calculator::something(), Some(42));
        // Sub
        assert_ok!(Calculator::do_sub(RuntimeOrigin::signed(1), 2));
        assert_eq!(Calculator::something(), Some(40));
        // Mul
        assert_ok!(Calculator::do_multiply(RuntimeOrigin::signed(1), 2));
        assert_eq!(Calculator::something(), Some(80));
        // Divide
        assert_ok!(Calculator::do_divide(RuntimeOrigin::signed(1), 2));
        assert_eq!(Calculator::something(), Some(40));
    });
}
