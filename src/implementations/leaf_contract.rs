use gstd::{msg , prelude::*, ActorId};
use io::*;

use crate::id_counter_mut;
use crate::projects_in_memory_mut;
use crate::update_state;

#[derive(Default)]
pub struct LeafContract {

}

impl LeafContract {
    pub fn say_hello(&mut self) {
        let _ = msg::reply(String::from("Hello world"), 0).expect("Error sending message to the contract.");
    }

    pub fn create_project(&mut self, owner: ActorId, name: String, description: String) {
        let  current_id = id_counter_mut();
        *current_id += 1;

        let project_id = current_id;

        let new_project = Project {
            owner,
            name,
            description
        };

        let projects_in_memory = projects_in_memory_mut();
        projects_in_memory.insert(*project_id, new_project);
        
        update_state();
        let _ = msg::reply(String::from("Proyecto creado exitosamente"), 0).expect("Error al momento de crear un proyecto");
    }
    
    pub fn update_project(&mut self, id: u128, name: String, description: String) {
        let projects_in_memory = projects_in_memory_mut();
        let project = projects_in_memory.get_mut(&id);
        
        if project.is_none() {
            let _ = msg::reply(String::from("Proyecto NO actualizado"), 0);
        }

        let project = project.unwrap();
        project.name = name;
        project.description = description;
        
        update_state();
        let _ = msg::reply(String::from("Proyecto actualizado"), 0);
    }
}
