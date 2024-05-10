use gstd::{msg, ActorId, String};

use io::{
    escrow_io::{EscrowAction, EscrowEvent}, EscrowId, Event
};

pub async fn donate(escrow_id: EscrowId, founder: ActorId) -> () {
    let reply = msg::send_for_reply_as::<EscrowAction, EscrowEvent>(
        escrow_id, 
        EscrowAction::Deposit { founder }, 
        msg::value(), 0
    ).expect("Error sending message to the contract").await.expect("Error receiving message from the contract");

    match reply {
        EscrowEvent::DeposidedCompleted { total_balance: _ } => {
            let _ = msg::reply(Event::Success, 0)
                .expect("Error sending message to the contract");
        },
        _ => {
            let _ = msg::reply(Event::Error { message: String::from("error") }, 0)
                .expect("Error sending message to the contract");
        }
    }
}

pub async fn aprove(escrow_id: EscrowId) -> () {
    let reply = msg::send_for_reply_as::<EscrowAction, EscrowEvent>(
        escrow_id, 
        EscrowAction::Aprove, 
        0, 0
    ).expect("Error sending message to the contract").await.expect("Error receiving message from the contract");

    match reply {
        EscrowEvent::AprovedCompleted => {
            let _ = msg::reply(Event::Success, 0)
                .expect("Error sending message to the contract");
        },
        _ => {
            let _ = msg::reply(Event::Error { message: String::from("error") }, 0)
                .expect("Error sending message to the contract");
        }
    }
}

pub async fn reject(escrow_id: EscrowId) -> () {
    let reply = msg::send_for_reply_as::<EscrowAction, EscrowEvent>(
        escrow_id, 
        EscrowAction::Reject, 
        0, 0
    ).expect("Error sending message to the contract").await.expect("Error receiving message from the contract");

    match reply {
        EscrowEvent::RejectedCompleted => {
            let _ = msg::reply(Event::Success, 0)
                .expect("Error sending message to the contract");
        },
        _ => {
            let _ = msg::reply(Event::Error { message: String::from("error") }, 0)
                .expect("Error sending message to the contract");
        }
    }
}