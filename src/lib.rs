#![no_std]
use gstd::{ msg , prelude::*,ActorId};
use io::*;
use implementations::leaf_contract::LeafContract;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod implementations;
mod handle;

pub static mut CONTRACT:Option<LeafContract> = None;

static mut STATE:Option<LeafContractState> = None;

fn contract_mut() -> &'static mut LeafContract {
    unsafe { CONTRACT.get_or_insert(Default::default()) }
}

fn state_mut() -> &'static mut LeafContractState {
    let state = unsafe { STATE.as_mut() };
    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init () {
    let _leaf_contract = LeafContract::default();

    let state = LeafContractState {
        projects: Vec::new(),
    };

    unsafe { STATE = Some(state) };
}

#[no_mangle]
extern "C" fn state() {
    let state = unsafe {
        STATE.get_or_insert(Default::default())
    };
    msg::reply(state, 0).expect("Failed to share state");
}