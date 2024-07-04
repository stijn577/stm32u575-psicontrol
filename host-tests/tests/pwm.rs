use functions::pwm::PwmDuty;

#[test]
fn test_calc() {
    let calc = PwmDuty::new(100);

    assert_eq!(calc.calc_duty(0), Ok(0));
    assert_eq!(calc.calc_duty(0), Ok(0));
    assert_eq!(calc.calc_duty(10), Ok(10));
    assert_eq!(calc.calc_duty(20), Ok(20));
    assert_eq!(calc.calc_duty(50), Ok(50));
    assert_eq!(calc.calc_duty(100), Ok(100));
    assert_eq!(calc.calc_duty(101), Err(()));
}
