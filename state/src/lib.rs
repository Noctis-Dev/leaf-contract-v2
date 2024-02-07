#![no_std]
use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};

#[metawasm]
pub mod metafns {

    pub type State = LeafContractState;
    
    pub fn search(state: State, name: String, description: String) -> Event {
        let projects = state.projects;
        // (u128, Project)
        let name_result: Vec<(u128, Project)> = projects.iter().filter(|&element| element.1.name == name).cloned().collect();
        
        if !name_result.is_empty() {
            return Event::ProjectsFound { projects: name_result }
        }
        return Event::ProjectsNotFound { message: String::from("Not Project was found") }
    }

    pub fn get_by_id(state: State, project_id: u128) -> Event {
        let projects = state.projects;
        
        if projects.is_empty() {
            return Event::ProjectsNotFound { message: String::from("Not Project was found") }
        }

        let possible_project: Option<&(u128, Project)> = projects.iter().filter(|&element| element.0 == project_id).next();

        if possible_project.is_none() {
            return Event::ProjectsNotFound { message: String::from("the project with id: ".to_owned() + &project_id.to_string()) }
        } else {
            return Event::ProjectFound { project: possible_project.unwrap().clone() }
        }
    }

}