#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct GateLinkCore;

#[contractimpl]
impl GateLinkCore {
    /// Initial test function to ensure the contract deploys successfully.
    /// Returns a simple heartbeat string.
    pub fn ping(env: Env) -> String {
        String::from_str(&env, "GateLink Contract Active")
    }
}

mod test;
