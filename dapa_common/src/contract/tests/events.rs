use crate::contract::ContractLog;

use super::*;

#[tokio::test]
async fn contract_event_flow() {
    // Create a contract that emits an event when called
    // Another contract that register an event listener from previous one
    // Call the first contract and verify the event is captured & well processed

    let code = r#"
        entry call_event() {
            emit_event(42, ["hello", "world!"]);
            return 0
        }
    "#;

    let mut chain_state = MockChainState::new();
    let emitter_hash = create_contract(&mut chain_state, code).expect("create emit event contract");

    let code = r#"
        fn on_contract_event(a: string, b: string) -> u64 {
            assert(a == "hello");
            assert(b == "world!");
            println(a + " " + b + " !");
            return 0
        }

        hook constructor() -> u64 {
            let contract_hash = Hash::from_hex("CONTRACT_HASH");
            let contract = Contract::new(contract_hash).expect("load contract");
            contract.listen_event(42, on_contract_event, 500);
            
            return 0
        }
    "#.replace("CONTRACT_HASH", &emitter_hash.to_string());

    // Deploy the listener hash
    let (_, execution) = deploy_contract(&mut chain_state, &code).await
        .expect("deploy listener contract");

    assert!(execution.is_success(), "listener contract deployment failed {:?}", execution);

    // Invoke the emitter contract to trigger the event
    let execution = invoke_contract(
        &mut chain_state,
        &emitter_hash,
        InvokeContract::Entry(0),
        vec![],
    ).await.expect("invoke emitter contract");

    assert!(execution.is_success(), "emitter contract execution failed {:?}", execution);

    let mut executions = 0;
    for (caller, logs) in chain_state.contract_logs {
        println!("Logs for contract caller {}:", caller);
        for log in logs {
            println!("- {:?}", log);
            match log {
                ContractLog::ExitCode(Some(0)) => {
                    executions += 1;
                },
                _ => {},
            }
        }
    }

    // - constructor execution
    // - call_event execution
    // - on_contract_event execution
    assert_eq!(executions, 3);
}