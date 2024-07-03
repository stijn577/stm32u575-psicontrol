//! Crate that has purely functional code, no hardware interaction, thus it can be tested on the host.

#![no_std]

pub fn add(a: usize, b: usize) -> usize {
    return a + b;
}
