#![no_std]
use gstd::{ prelude::*, ActorId, CodeId };
use gmeta::{In, InOut, Metadata, Out};

pub type MilestoneId = u8;

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Action {
    HelloAction,
    CreateProject {
        name: String,
        description: String,
        milestone_value: Vec<(u128, MilestoneId)>,
    },
    UpdateProject {
        id: ActorId,
        name: String,
        description: String
    },
    Donate {
        scrow_id: ActorId
    },
    Aprove {
        scrow_id: ActorId
    },
    Reject {
        scrow_id: ActorId
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum  Event {
    Success,
    ProjectCreated {
        project: Project
    },
    ProjectsFound {
        projects: Vec<(ActorId, Project)>
    },
    ProjectsNotFound {
        message: String
    },
    ProjectFound {
        project: (u128, Project)
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ScrowAction {
    Deposit {
        founder: ActorId
    },
    Aprove,
    Reject,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ScrowEvent {
    DeposidedCompleted {
        total_balance: u128,
    },
    AprovedCompleted,
    RejectedCompleted,
    Error {
        message: String
    }
}

#[derive(TypeInfo, Encode, Decode, Default)]
pub struct InitScrow {
    pub collector: ActorId,
    pub max_milestone_number: MilestoneId
}

#[derive(Default, Decode, Clone, TypeInfo, Encode, Debug)]
pub struct Project {
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub scrow_id: ActorId,
    pub milestone_value: Vec<(u128, MilestoneId)>,
}

#[derive(Default, Decode, Clone, TypeInfo, Encode)]
pub struct LeafContractState {
    pub projects: Vec<(ActorId, Project)>
}

pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
    type Init = In<CodeId>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply=();
    type Signal = ();
    type State = Out<LeafContractState>;
}