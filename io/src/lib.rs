#![no_std]
use escrow_io::{EscrowAction, EscrowEvent};
use gstd::{ prelude::*, ActorId, CodeId };
use gmeta::{In, InOut, Metadata, Out};

pub mod escrow_io;

pub type MilestoneId = u8;
pub type EscrowId = ActorId;

#[derive(TypeInfo, Encode, Decode, Clone)]
pub enum Action {
    CreateProject {
        name: String,
        description: String,
        milestones: Vec<Milestone>,
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

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum  Event {
    Success,
    ProjectCreated {
        project: Project
    },
    ProjectsNotFound {
        message: String
    },
    Error {
        message: String
    }
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct Milestone {
    pub title: String,
    pub value: u128,
}

#[derive(Default, TypeInfo, Encode, Decode, Clone)]
pub struct Project {
    pub escrow_id: EscrowId,
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub milestone_value: Vec<Milestone>,
}

#[derive(Default, TypeInfo, Encode, Decode, Clone)]
pub struct LeafContractState {
    pub projects: Vec<Project>
}

pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
    type Init = In<CodeId>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = InOut<EscrowAction, EscrowEvent>;
    type Signal = ();
    type State = Out<LeafContractState>;
}