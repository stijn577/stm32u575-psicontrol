#[macro_export]
macro_rules! log {
    ($logger:expr) => {
        #[cfg(feature = "defmt")]
        $logger
    };
}
