use core::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use serde::{Deserialize, Serialize};

pub type PWBuff = heapless::String<128>;

#[derive(Clone)]
pub struct Encrypted;
#[derive(Clone)]
pub struct Unencrypted;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash)]
pub struct Password<T> {
    pwbuff: PWBuff,
    state: PhantomData<T>,
}

impl<T> Password<T> {
    pub fn new(value: PWBuff) -> Password<Unencrypted> {
        Password::<Unencrypted> { pwbuff: value, state: PhantomData }
    }
}

impl Password<Unencrypted> {
    pub fn encrypt(self) -> Password<Encrypted> {
        // just a function to show typestate
        Password::<Encrypted> {
            pwbuff: self.pwbuff,
            state: PhantomData,
        }
    }
}

impl Password<Encrypted> {
    pub fn decrypt(self) -> Password<Unencrypted> {
        // just a pseudocode function to show typestate
        Password::<Unencrypted> {
            pwbuff: self.pwbuff,
            state: PhantomData,
        }
    }
}

impl Deref for Password<Unencrypted> {
    type Target = PWBuff;

    fn deref(&self) -> &Self::Target {
        &self.pwbuff
    }
}

impl DerefMut for Password<Unencrypted> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pwbuff
    }
}
