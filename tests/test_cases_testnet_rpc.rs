//! Runs standard test cases against TestNet with several nodes running in separate threads.
//! The communication is performed through `RPCUser` that uses the standard RPC API to communicate.
#[cfg(feature = "expensive_tests")]
#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;

    use near_primitives::test_utils::init_test_module_logger;
    use testlib::node::{create_nodes_from_seeds, Node, NodeConfig, ThreadNode};
    use testlib::runtime_utils::alice_account;
    use testlib::standard_test_cases::*;
    use testlib::test_helpers::heavy_test;

    fn create_thread_nodes_rpc() -> Vec<ThreadNode> {
        init_test_module_logger("runtime");
        let nodes = create_nodes_from_seeds(vec![
            "alice.near".to_string(),
            "bob.near".to_string(),
            "carol.near".to_string(),
            "dan.near".to_string(),
        ]);
        let mut nodes: Vec<_> = nodes
            .into_iter()
            .map(|cfg| match cfg {
                NodeConfig::Thread(config) => ThreadNode::new(config),
                _ => unreachable!(),
            })
            .collect();
        let account_names: Vec<_> = nodes.iter().map(|node| node.account_id().unwrap()).collect();

        assert_eq!(account_names[0], alice_account());
        for i in 0..nodes.len() {
            nodes[i].start();
        }
        // Let the nodes boot up a bit.
        for _ in 0..100 {
            let height = nodes[0].user().get_best_height();
            if height.is_some() && height.unwrap() > 1 {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }

        nodes
    }

    /// Macro for running testnet tests using ThreadNode and RPCUser.
    /// Guard each test with heavy_test mutex.
    macro_rules! run_testnet_test {
        ($f:expr) => {
            heavy_test(|| {
                let mut nodes = create_thread_nodes_rpc();
                let node = nodes.remove(0);
                $f(node)
            });
        };
    }

    #[test]
    fn test_smart_contract_simple_testnet() {
        run_testnet_test!(test_smart_contract_simple);
    }

    #[test]
    fn test_smart_contract_self_call_testnet() {
        run_testnet_test!(test_smart_contract_self_call);
    }

    #[test]
    fn test_smart_contract_bad_method_name_testnet() {
        run_testnet_test!(test_smart_contract_bad_method_name);
    }

    #[test]
    fn test_smart_contract_empty_method_name_with_no_tokens_testnet() {
        run_testnet_test!(test_smart_contract_empty_method_name_with_no_tokens);
    }

    #[test]
    fn test_smart_contract_empty_method_name_with_tokens_testnet() {
        run_testnet_test!(test_smart_contract_empty_method_name_with_tokens);
    }

    #[test]
    fn test_smart_contract_with_args_testnet() {
        run_testnet_test!(test_smart_contract_with_args);
    }

    #[test]
    fn test_nonce_update_when_deploying_contract_testnet() {
        run_testnet_test!(test_nonce_update_when_deploying_contract);
    }

    #[test]
    fn test_nonce_updated_when_tx_failed_testnet() {
        run_testnet_test!(test_nonce_updated_when_tx_failed);
    }

    #[test]
    fn test_upload_contract_testnet() {
        run_testnet_test!(test_upload_contract);
    }

    #[test]
    fn test_redeploy_contract_testnet() {
        run_testnet_test!(test_redeploy_contract);
    }

    #[test]
    fn test_send_money_testnet() {
        run_testnet_test!(test_send_money);
    }

    #[test]
    fn test_send_money_over_balance_testnet() {
        run_testnet_test!(test_send_money_over_balance);
    }

    #[test]
    fn test_refund_on_send_money_to_non_existent_account_testnet() {
        run_testnet_test!(test_refund_on_send_money_to_non_existent_account);
    }

    #[test]
    fn test_create_account_testnet() {
        run_testnet_test!(test_create_account);
    }

    #[test]
    fn test_create_account_again_testnet() {
        run_testnet_test!(test_create_account_again);
    }

    #[test]
    fn test_create_account_failure_invalid_name_testnet() {
        run_testnet_test!(test_create_account_failure_invalid_name);
    }

    #[test]
    fn test_create_account_failure_already_exists_testnet() {
        run_testnet_test!(test_create_account_failure_already_exists);
    }

    #[test]
    fn test_swap_key_testnet() {
        run_testnet_test!(test_swap_key);
    }

    #[test]
    fn test_add_access_key_function_call_testnet() {
        run_testnet_test!(test_add_access_key_function_call);
    }

    #[test]
    fn test_add_existing_key_testnet() {
        run_testnet_test!(test_add_existing_key);
    }

    #[test]
    fn test_delete_key_testnet() {
        run_testnet_test!(test_delete_key);
    }

    #[test]
    fn test_delete_key_not_owned_testnet() {
        run_testnet_test!(test_delete_key_not_owned);
    }

    #[test]
    fn test_delete_key_last_testnet() {
        run_testnet_test!(test_delete_key_last);
    }

    #[test]
    fn test_add_key_testnet() {
        run_testnet_test!(test_add_key);
    }

    #[test]
    fn test_delete_access_key_testnet() {
        run_testnet_test!(test_delete_access_key);
    }

    #[test]
    fn test_add_access_key_with_allowance_testnet() {
        run_testnet_test!(test_add_access_key_with_allowance);
    }

    #[test]
    fn test_delete_access_key_with_allowance_testnet() {
        run_testnet_test!(test_delete_access_key_with_allowance);
    }

    #[test]
    fn test_access_key_smart_contract_testnet() {
        run_testnet_test!(test_access_key_smart_contract);
    }
}
