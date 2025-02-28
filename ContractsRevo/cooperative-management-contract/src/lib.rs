#![no_std]

use datatype::DataKey;
use soroban_sdk::{Address, Env, contract, contractimpl};

mod datatype;
mod governance;
mod interface;
mod membership;
mod profit_distribution;
mod resource_sharing;

#[contract]
pub struct CooperativeManagementContract;

#[contractimpl]
impl CooperativeManagementContract {
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("Contract is already initialized");
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }
}
