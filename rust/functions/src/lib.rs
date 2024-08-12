//! Crate that has purely functional code, no hardware interaction, thus it can be tested on the host.
//! We can't test stuff on the host in this crate itself because it needs to declare `#![no_std]`,
//! this means that we lose access to the standard library testing harness. The reason we still seperate this is so we can make
//! an std crate that will use only the functional parts of the code to make testing easier/faster. For that we are not allowed to use
//! any of the embedded functionality or code made for other architectures (ARM Cortex-M in this case).
#![no_std]
#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "embassy", feature = "alloc", feature = "serde"))]
pub mod embassy_trait_impls;
pub mod macros;
pub mod messages;
pub mod password;
pub mod pwm;
#[cfg(feature = "std")]
pub mod std_trait_impls;
