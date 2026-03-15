#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_ping() {
    let env = Env::default();
    let contract_id = env.register(GateLinkCore, ());
    let client = GateLinkCoreClient::new(&env, &contract_id);

    let result = client.ping();
    assert_eq!(result, String::from_str(&env, "GateLink Contract Active"));
}