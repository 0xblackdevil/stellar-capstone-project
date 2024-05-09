#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(&env, &contract_id);

    assert_eq!(
        client.register_user(&symbol_short!("john")),
        User {
            user_id: env.ledger().timestamp(),
            user_name: symbol_short!("john"),
        }
    );

    assert_eq!(
        client.register_user(&symbol_short!("bob")),
        User {
            user_id: env.ledger().timestamp(),
            user_name: symbol_short!("bob"),
        }
    );
}
