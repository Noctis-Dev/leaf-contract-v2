#![no_std]
use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};

#[metawasm]
pub mod metafns {
    use gstd::ActorId;


    pub type State = LeafContractState;
    
    pub fn search(state: State, name: String, description: String) -> Event {
        let projects = state.projects;
        let mut name_result: Vec<(u128, Project)> = Vec::new();
        let mut description_result: Vec<(u128, Project)> = Vec::new();

        let description_lower = description.to_lowercase();

        if !name.is_empty() {
            name_result = projects.iter().filter(|&element| element.1.name == name).cloned().collect();
        }
        if !description.is_empty() {
            description_result = projects.iter().filter(|&element| element.1.description.to_lowercase().contains(&description_lower)).cloned().collect();
        }

        let projects_found = [name_result, description_result].concat();

        if !projects_found.is_empty() {
            return Event::ProjectsFound { projects: projects_found };
        }
        
        return Event::ProjectsNotFound { message: String::from("No project was found") };
    }

    pub fn get_by_id(state: State, project_id: ActorId) -> Event {
        let projects = state.projects;
        
        if projects.is_empty() {
            return Event::ProjectsNotFound { message: String::from("Not Project was found") }
        }

        let possible_project: Option<&(ActorId, Project)> = projects.iter().filter(|&element| element.0 == project_id).next();

        if possible_project.is_none() {
            return Event::ProjectsNotFound { message: String::from("the project with id: ".to_owned() + &project_id.to_string()) }
        } else {
            return Event::ProjectFound { project: possible_project.unwrap().clone() }
        }
    }
}