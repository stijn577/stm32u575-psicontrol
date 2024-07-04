pub struct PwmDuty(u16);

impl PwmDuty {
    pub fn new(max_duty: u16) -> Self {
        Self(max_duty)
    }

    pub fn calc_duty(&self, percent: u16) -> Result<u16, ()> {
        // early return error if value passed in is not readable as a percent
        if !(0..=100).contains(&percent) {
            return Err(());
        }

        Ok(self.0 / 100 * percent) // divide first, overflow detected and panics in debug, but not in release
    }
}
