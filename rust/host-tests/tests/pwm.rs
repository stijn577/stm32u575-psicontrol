use functions::pwm::PwmDuty;

#[test]
fn test_calc() {
    let pwmduty = PwmDuty::new(100);

    assert_eq!(pwmduty.calc(0), Some(0));
    assert_eq!(pwmduty.calc(0), Some(0));
    assert_eq!(pwmduty.calc(10), Some(10));
    assert_eq!(pwmduty.calc(20), Some(20));
    assert_eq!(pwmduty.calc(50), Some(50));
    assert_eq!(pwmduty.calc(100), Some(100));
    assert_eq!(pwmduty.calc(101), None);
}
