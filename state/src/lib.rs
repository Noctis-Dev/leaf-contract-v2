#![no_std]
use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    use io::LeafContractState;

    pub type State = <LeafContractState as Metadata>::State;

    // Add your State functions
    pub fn state(state: State) -> LeafContractState {
        state
    }
}