//! Crate that has purely functional code, no hardware interaction, thus it can be tested on the host.
//! We can't test stuff on the host in this crate itself because it needs to declare `#![no_std]`,
//! this means that we lose access to the standard library testing harness.

#![no_std]

pub mod pwm;

pub fn add(a: usize, b: usize) -> usize {
    a + b
}
