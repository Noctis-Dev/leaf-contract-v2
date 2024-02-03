use gstd::{ msg , prelude::*};
use io::*;

use crate::contract_mut;

#[no_mangle]
extern "C" fn handle(){
    let action: Action = msg::load().expect("Could not load Action");
    let source = msg::source();

    let contract = contract_mut();

    match &action {
        Action::HelloAction => {
            contract.say_hello()
        }

        Action::CreateProject { name, description } => {
            contract.create_project(source, name.to_string(), description.to_string());
        }

        Action::UpdateProject { id, name, description } => {
            contract.update_project(*id, name.to_string(), description.to_string());
        }
    };
}