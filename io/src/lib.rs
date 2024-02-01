#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{Out, InOut,Metadata};

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    HelloAction,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum  Event {
    ActionEvent,
}

#[derive(Default, Decode, Clone, TypeInfo, Encode)]
pub struct LeafContractState {
    pub projects: Vec<(u32, String)>
}

pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<Action,String>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Out<LeafContractState>;
}