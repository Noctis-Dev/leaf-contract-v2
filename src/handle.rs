use gstd::{ async_main, msg, prelude::*};
use io::*;

use crate::contract_mut;

#[async_main]
async fn main(){
    let action: Action = msg::load().expect("Could not load Action");
    let source = msg::source();

    let contract = contract_mut();

    match &action {
        Action::CreateProject { name, description, milestones } => {
            contract.create_project(source, name.to_string(), description.to_string(), milestones.clone()).await;
        }
        Action::UpdateProject { id, name, description } => {
            contract.update_project(*id, name.to_string(), description.to_string()).await;
        }
        Action::Donate { scrow_id } => {
            contract.donate(scrow_id.clone(), source).await;
        },
        Action::Aprove { scrow_id } => {
            contract.aprove(scrow_id.clone()).await;
        },
        Action::Reject { scrow_id } => {
            contract.aprove(scrow_id.clone()).await;
        },
    };
}