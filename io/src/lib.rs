#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{Out, InOut,Metadata};

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Action {
    HelloAction,
    CreateProject {
        name: String,
        description: String
    },
    UpdateProject {
        id: u128,
        name: String,
        description: String
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum  Event {
    ActionEvent,
    ProjectsFound {
        projects: Vec<(u128, Project)>
    },
    ProjectsNotFound {
        message: String
    },
    ProjectFound {
        project: (u128, Project)
    }
}

#[derive(Default, Decode, Clone, TypeInfo, Encode, Debug)]
pub struct Project {
    pub owner: ActorId,
    pub name: String,
    pub description: String
}

#[derive(Default, Decode, Clone, TypeInfo, Encode)]
pub struct LeafContractState {
    pub projects: Vec<(u128, Project)>
}

pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
     type Init = ();
     type Handle = InOut<Action, String>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Out<LeafContractState>;
}