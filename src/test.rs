#![cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);

        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.get(), 2);
    }

    #[test]
    fn test_decrement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ContadorContract);
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
        let contract_id = env.register_contract(None, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);

        client.increment();
        client.increment();
        client.reset();
        assert_eq!(client.get(), 0);
    }

    #[test]
    fn test_get_initial_value() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);

        assert_eq!(client.get(), 0);
    }
}
