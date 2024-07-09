pub struct PwmDuty(u32);

impl PwmDuty {
    pub fn new(max_duty: u32) -> Self {
        Self(max_duty)
    }

    pub fn calc(&self, percent: u32) -> Option<u32> {
        // early return error if value passed in is not readable as a percent
        if !(0..=100).contains(&percent) {
            return None;
        }

        Some(self.0 / 100 * percent) // divide first, overflow detected and panics in debug, but not in release
    }
}
