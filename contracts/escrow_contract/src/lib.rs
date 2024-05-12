#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, Address, Env, Symbol, Vec,
};

#[contracttype]
pub enum RegisteredUsers {
    User(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub user_id: u64,
    pub submitted: Vec<u64>,
    pub accepted: Vec<u64>,
}

#[contracttype]
pub enum ListOfProjects {
    Project(u64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Project {
    pub p_id: u64,
    pub p_name: Symbol,
    pub p_des: Symbol,
    pub p_fund: u64,
    pub client: u64,
    pub milestone: Vec<Milestone>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub m_name: Symbol,
    pub m_des: Symbol,
    pub m_price: u64,
    pub m_deadline: u64,
    pub m_status: Symbol, // pending, accepted, rejected
    pub m_accepter: Address,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn register_user(e: Env, _user_address: Address) -> User {
        let mut user: User = Self::view_user(e.clone(), _user_address.clone());

        if user.user_id != 0 {
            panic!("User already exists");
        } else {
            user.user_id = e.ledger().timestamp();

            e.storage()
                .persistent()
                .set(&RegisteredUsers::User(_user_address.clone()), &user);
        }
        log!(&e, "User Registered: {}", _user_address);
        user
    }

    pub fn view_user(e: Env, _user_address: Address) -> User {
        let key: RegisteredUsers = RegisteredUsers::User(_user_address.clone());
        e.storage().instance().get(&key).unwrap_or(User {
            user_id: 0,
            submitted: Vec::new(&e),
            accepted: Vec::new(&e),
        })
    }

    pub fn create_project(
        e: Env,
        _p_id: u64,
        _p_name: Symbol,
        _p_des: Symbol,
        _p_fund: u64,
        _client: u64,
    ) -> Project {
        let mut project: Project = Self::view_project(e.clone(), _p_id.clone());

        if project.p_id != 0 {
            panic!("Project already exists");
        } else {
            project.p_name = _p_name.clone();
            project.p_des = _p_des.clone();
            project.p_fund = _p_fund;
            project.client = _client.clone();

            e.storage()
                .persistent()
                .set(&ListOfProjects::Project(_p_id.clone()), &project);

            log!(&e, "Project Added: {} by {}", _p_name, _client);
        }

        project
    }

    pub fn view_project(e: Env, _p_id: u64) -> Project {
        let key: ListOfProjects = ListOfProjects::Project(_p_id.clone());

        e.storage().instance().get(&key).unwrap_or(Project {
            p_id: 0,
            p_name: symbol_short!("Not_found"),
            p_des: symbol_short!("Not_found"),
            p_fund: 0,
            client: 0,
            milestone: Vec::new(&e),
        })
    }

    pub fn accept_project(e: Env) {}

    // pub fn submit_milestone() {}

    // pub fn accept_milestone() {}

    // pub fn release_fund() {}

    // pub fn raise_dispute() {}

    // pub fn resolve_dispute() {}
}
mod test;
