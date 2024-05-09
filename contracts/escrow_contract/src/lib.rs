#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Env, Symbol};

#[contracttype]
pub enum RegisteredUsers {
    User(Symbol),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub user_id: u64,
    pub user_name: Symbol,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn register_user(e: Env, _user_name: Symbol) -> User {
        let mut user: User = Self::view_user(e.clone(), _user_name.clone());

        if user.user_id != 0 {
            panic!("User already exists");
        } else {
            user.user_id = e.ledger().timestamp();
            user.user_name = _user_name.clone();

            e.storage()
                .persistent()
                .set(&RegisteredUsers::User(_user_name.clone()), &user);
        }

        log!(&e, "User Registered: {}", _user_name);

        user
    }

    pub fn view_user(e: Env, _user_name: Symbol) -> User {
        let key: RegisteredUsers = RegisteredUsers::User(_user_name.clone());
        e.storage().instance().get(&key).unwrap_or(User {
            user_id: 0,
            user_name: symbol_short!("not_found"),
        })
    }

    pub fn create_project(e: Env, _total_budget: u32) {
        log!(&e, "Projects Total Budget: {}", _total_budget);
    }

    // pub fn fund_project() {}

    // pub fn accept_project() {}

    // pub fn submit_milestone() {}

    // pub fn accept_milestone() {}

    // pub fn release_fund() {}

    // pub fn raise_dispute() {}

    // pub fn resolve_dispute() {}
}

mod test;
