use gstd::prog::ProgramGenerator;
use gstd::{msg , prelude::*, ActorId, CodeId};
use io::*;

use crate::projects_in_memory_mut;
use crate::update_state;

#[derive(Default)]
pub struct LeafContract {
    pub program_id: CodeId,
}

impl LeafContract {

    pub fn say_hello(&mut self) {
        let _ = msg::reply(String::from("Hello world"), 0).expect("Error sending message to the contract.");
    }

    pub fn create_project(&mut self, owner: ActorId, name: String, description: String, milestones: Vec<(u128, MilestoneId)>) {
        let scrow_id = ProgramGenerator::create_program_with_gas(
            self.program_id,
            InitScrow {
                collector: owner,
                max_milestone_number: milestones.len() as MilestoneId
            },
            10_000_000_000,
            0,
        ).expect("Error during scrow creation");

        let project_id = scrow_id.1.clone();

        let new_project = Project {
            owner,
            name,
            description,
            scrow_id: scrow_id.1,
            milestone_value: milestones
        };

        let projects_in_memory = projects_in_memory_mut();
        projects_in_memory.insert(project_id, new_project.clone());
        
        update_state();
        let _ = msg::reply(Event::ProjectCreated { project: new_project.clone() }, 0).expect("Error al momento de crear un proyecto");
    }
    
    pub fn update_project(&mut self, id: ActorId, name: String, description: String) {
        let projects_in_memory = projects_in_memory_mut();
        let project = projects_in_memory.get_mut(&id);
        
        if project.is_none() {
            let _ = msg::reply(String::from("Proyecto NO actualizado"), 0);
        }

        let project = project.unwrap();
        project.name = name;
        project.description = description;
        
        update_state();
        let _ = msg::reply(Event::Success, 0);
    }

    pub fn donate(&self, scrow_id: ActorId) {
        let projects_in_memory = projects_in_memory_mut();
        let project: Option<Project> = projects_in_memory.get(&scrow_id).cloned();

        if project.is_none() {
            let _ = msg::reply(Event::ProjectsNotFound { message: String::from("project not found") }, 0);
        }

        let _ = msg::send(
            project.unwrap().clone().scrow_id, 
            ScrowAction::Deposit { founder: msg::source() }, 
            msg::value()
        ).expect("Error during sending a message");
        
        let _ = msg::reply(Event::Success, 0);
    }

    pub fn aprove(&self, scrow_id: ActorId) {
        let projects_in_memory = projects_in_memory_mut();
        let project: Option<Project> = projects_in_memory.get(&scrow_id).cloned();

        if project.is_none() {
            let _ = msg::reply(Event::ProjectsNotFound { message: String::from("project not found") }, 0);
        }

        let _ = msg::send(
            project.unwrap().clone().scrow_id, 
            ScrowAction::Aprove, 
            0
        ).expect("Error during sending a message");

        let _ = msg::reply(Event::Success, 0);
    }

    pub fn reject(&self, scrow_id: ActorId) {
        let projects_in_memory = projects_in_memory_mut();
        let project: Option<Project> = projects_in_memory.get(&scrow_id).cloned();

        if project.is_none() {
            let _ = msg::reply(Event::ProjectsNotFound { message: String::from("project not found") }, 0);
        }

        let _ = msg::send(
            project.unwrap().clone().scrow_id, 
            ScrowAction::Reject, 
            0
        ).expect("Error during sending a message");

        let _ = msg::reply(Event::Success, 0);
    }
}
