use gstd::{
    ActorId, 
    Decode, 
    Encode, 
    String, 
    TypeInfo
};

use crate::MilestoneId;

#[derive(TypeInfo, Encode, Decode, Default)]
pub struct InitEscrow {
    pub collector: ActorId,
    pub max_milestone_number: MilestoneId
}

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EscrowAction {
    Deposit {
        founder: ActorId
    },
    Aprove,
    Reject,
}

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EscrowEvent {
    DeposidedCompleted {
        total_balance: u128,
    },
    AprovedCompleted,
    RejectedCompleted,
    Error {
        message: String
    }
}