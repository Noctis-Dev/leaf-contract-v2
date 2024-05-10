#![no_std]
use gstd::{ msg, prelude::*, ActorId, CodeId };
use hashbrown::HashMap;
use io::*;
use contract::LeafContract;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod escrow_contract;
mod contract;
mod handle;

pub static mut CONTRACT: Option<LeafContract> = None;
pub static mut STATE: Option<LeafContractState> = None;
pub static mut PROJECTS_IN_MEMORY: Option<HashMap<ActorId, Project>> = None;

pub fn contract_mut() -> &'static mut LeafContract {
    unsafe { CONTRACT.get_or_insert(LeafContract::default()) }
}

pub fn state_mut() -> &'static mut LeafContractState {
    let state = unsafe { STATE.as_mut() };
    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init () {

    let scrow_code_hash: CodeId = msg::load().expect("Unable to decode the incoming message");

    let state = LeafContractState {
        projects: Vec::new(),
    };

    unsafe { 
        CONTRACT = Some(LeafContract {
            program_id: scrow_code_hash,
        }); 
        PROJECTS_IN_MEMORY = Some(HashMap::new()); 
        STATE = Some(state) 
    };
}

// TODO traer proyectos
#[no_mangle]
extern "C" fn state() {
    let state = state_mut();
    msg::reply(state, 0).expect("Failed to share state");
}