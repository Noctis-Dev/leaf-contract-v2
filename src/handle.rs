use gstd::{ msg , prelude::*,ActorId};
use io::*;

use crate::LeafContract;
use crate::CONTRACT;

#[no_mangle]
extern "C" fn handle(){
    let action: Action = msg::load().expect("Could not load Action");

    let contract = unsafe { CONTRACT.get_or_insert(LeafContract::default()) };

    match &action {
        Action::HelloAction => {
            LeafContract::say_hello();
        }
    };
}