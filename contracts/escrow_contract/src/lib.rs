#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectStatus {
    Active,
    Completed,
    Disputed,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn create_project(e: Env, _total_budget: u32) {}

    pub fn fund_project() {}

    pub fn accept_project() {}

    pub fn submit_milestone() {}

    pub fn accept_milestone() {}

    pub fn release_fund() {}

    pub fn raise_dispute() {}

    pub fn resolve_dispute() {}
}
