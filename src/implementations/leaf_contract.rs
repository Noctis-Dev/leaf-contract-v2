#![no_std]
use gstd::{msg , prelude::*, ActorId};

#[derive(Default)]
pub struct LeafContract;

impl LeafContract {
    pub fn say_hello() {
        let _ = msg::reply(String::from("Hello world"), 0).expect("Error sending message to the contract.");
    }
}
