#![no_std]
use gstd::{ msg, prelude::*, ActorId, CodeId };
use hashbrown::HashMap;
use io::*;
use implementations::leaf_contract::LeafContract;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod implementations;
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

pub fn projects_in_memory_mut() -> &'static mut HashMap<ActorId, Project> {
    unsafe { PROJECTS_IN_MEMORY.get_or_insert(HashMap::new()) }
}

pub fn update_state() {
    let state = state_mut();
    let memory_projects = projects_in_memory_mut();
    let projects: Vec<(ActorId, io::Project)> = memory_projects.into_iter().map(|(k, v)| (*k, v.clone())).collect();
    state.projects = projects;
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