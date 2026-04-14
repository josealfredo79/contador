#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct ContadorContract;

#[contractimpl]
impl ContadorContract {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        count
    }

    pub fn decrement(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let new_count = count.saturating_sub(1);
        env.storage().instance().set(&COUNTER, &new_count);
        new_count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }

    pub fn reset(env: Env) {
        env.storage().instance().set(&COUNTER, &0u32);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register(ContadorContract, ());
        let client = ContadorContractClient::new(&env, &contract_id);

        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.get(), 2);
    }

    #[test]
    fn test_decrement() {
        let env = Env::default();
        let contract_id = env.register(ContadorContract, ());
        let client = ContadorContractClient::new(&env, &contract_id);

        client.increment();
        client.increment();
        client.increment();
        assert_eq!(client.get(), 3);
        assert_eq!(client.decrement(), 2);
        assert_eq!(client.decrement(), 1);
        assert_eq!(client.decrement(), 0);
        assert_eq!(client.decrement(), 0);
    }

    #[test]
    fn test_reset() {
        let env = Env::default();
        let contract_id = env.register(ContadorContract, ());
        let client = ContadorContractClient::new(&env, &contract_id);

        client.increment();
        client.increment();
        client.reset();
        assert_eq!(client.get(), 0);
    }

    #[test]
    fn test_get_initial_value() {
        let env = Env::default();
        let contract_id = env.register(ContadorContract, ());
        let client = ContadorContractClient::new(&env, &contract_id);

        assert_eq!(client.get(), 0);
    }
}
