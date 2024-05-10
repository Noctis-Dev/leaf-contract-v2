use gstd::{
    msg, 
    prog::ProgramGenerator, 
    ActorId, 
    CodeId, 
    String, 
    Vec
};

use io::{
    escrow_io::InitEscrow, 
    EscrowId, 
    Event, 
    LeafContractState,
    Milestone, 
    MilestoneId, 
    Project
};

use crate::state_mut;
use crate::escrow_contract::escrow_calls;

#[derive(Default)]
pub struct LeafContract {
    pub program_id: CodeId,
}

impl LeafContract {

    pub async fn create_project(&mut self, owner: ActorId, name: String, description: String, milestones: Vec<Milestone>) {
        let scrow_id = ProgramGenerator::create_program_with_gas(
            self.program_id, 
            InitEscrow {
                collector: owner,
                max_milestone_number: milestones.len() as MilestoneId
            },
            10_000_000_000, 
            0
        ).expect("Error during the program creation");

        let project_id: EscrowId = scrow_id.1.clone();

        let new_project: Project = Project {
            owner, 
            name,
            description,
            escrow_id: project_id.clone(),
            milestone_value: milestones
        };

        let state = state_mut();
        state.projects.push(new_project.clone());

        let _ = msg::reply(Event::ProjectCreated { project: new_project.clone() }, 0).expect("Error sending message to the contract");
    }

    pub async fn update_project(&mut self, escrow_id: EscrowId, name: String, description: String) {
        let state = state_mut();
        let project = state.projects.iter_mut().find(|project| project.escrow_id == escrow_id);

        if project.is_none() {
            let _ = msg::reply(Event::ProjectsNotFound { message: String::from("Project not found") }, 0)
                .expect("Error sending message to the contract");
        }

        let project = project.unwrap();
        project.name = name;
        project.description = description;

        let _ = msg::reply(Event::Success, 0).expect("Error sending message to the contract");
    }

    pub async fn donate(&mut self, escrow_id: EscrowId, founder: ActorId) {
        let state: &mut LeafContractState = state_mut();

        if let Some(_project) = state.projects.iter_mut().find(|project| project.escrow_id == escrow_id) {
            escrow_calls::donate(escrow_id.clone(), founder.clone()).await;
        } 

        let _ = msg::reply(Event::ProjectsNotFound { message: String::from("Project not found") }, 0)
            .expect("Error sending message to the contract");
    }

    pub async fn aprove(&mut self, escrow_id: EscrowId) {
        let state: &mut LeafContractState = state_mut();

        if let Some(_project) = state.projects.iter_mut().find(|project| project.escrow_id == escrow_id) {
            escrow_calls::aprove(escrow_id.clone()).await;
        } 

        let _ = msg::reply(Event::ProjectsNotFound { message: String::from("Project not found") }, 0)
            .expect("Error sending message to the contract");
    }

    pub async fn reject(&mut self, escrow_id: EscrowId) {
        let state: &mut LeafContractState = state_mut();

        if let Some(_project) = state.projects.iter_mut().find(|project| project.escrow_id == escrow_id) {
            escrow_calls::reject(escrow_id.clone()).await;
        } 

        let _ = msg::reply(Event::ProjectsNotFound { message: String::from("Project not found") }, 0)
            .expect("Error sending message to the contract");
    }
}