use std::time::Duration;

use dharitri_sc_scenario::dharitri_sc::types::FunctionCall;
#[allow(unused_imports)]
use dharitri_sc_snippets::dharitri_sc::types::{
    DctTokenPayment, MultiValueEncoded, TokenIdentifier,
};
use dharitri_sc_snippets::{
    dharitri_sc::types::{ContractCall, ContractCallNoPayment},
    dharitri_sc_scenario::{
        mandos_system::ScenarioRunner, scenario_format::interpret_trait::InterpretableFrom,
        standalone::retrieve_account_as_scenario_set_state,
    },
};

use super::*;

const WMOA_SWAP_SC_BECH32: &str = "erd1qqqqqqqqqqqqqpgqcy2wua5cq59y6sxqj2ka3scayh5e5ms7cthqht8xtp";
const WMOA_TOKEN_IDENTIFIER: &str = "WMOA-6cf38e";
const WRAP_AMOUNT: u64 = 50000000000000000; // 0.05 MOA
const UNWRAP_AMOUNT: u64 = 25000000000000000; // 0.025 WMOA

impl MultisigInteract {
    pub async fn wmoa_swap_full(&mut self) {
        self.deploy().await;
        self.feed_contract_moa().await;
        self.wrap_moa().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.unwrap_moa().await;
    }

    pub async fn wrap_moa(&mut self) {
        println!("proposing wrap moa...");
        let action_id = self.propose_wrap_moa().await;

        println!("perfoming wrap moa action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn unwrap_moa(&mut self) {
        println!("proposing unwrap moa...");
        let action_id = self.propose_unwrap_moa().await;

        println!("perfoming unwrap moa action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn wmoa_swap_set_state(&mut self) {
        let scenario_raw = retrieve_account_as_scenario_set_state(
            Config::load_config().gateway().to_string(),
            WMOA_SWAP_SC_BECH32.to_string(),
            true,
        )
        .await;

        let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

        self.interactor.pre_runners.run_scenario(&scenario);
        self.interactor.post_runners.run_scenario(&scenario);
    }

    async fn propose_wrap_moa(&mut self) -> usize {
        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        bech32::decode(WMOA_SWAP_SC_BECH32),
                        WRAP_AMOUNT,
                        FunctionCall::new("wrapMoa"),
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed wrap moa action `{action_id}`");
        action_id
    }

    async fn propose_unwrap_moa(&mut self) -> usize {
        let contract_call = ContractCallNoPayment::<StaticApi, ()>::new(
            bech32::decode(WMOA_SWAP_SC_BECH32).into(),
            "unwrapMoa",
        )
        .with_dct_transfer(DctTokenPayment::new(
            TokenIdentifier::from(WMOA_TOKEN_IDENTIFIER),
            0u64,
            UNWRAP_AMOUNT.into(),
        ))
        .into_normalized();

        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        contract_call.basic.to,
                        0u64,
                        contract_call.basic.function_call,
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed unwrap moa action `{action_id}`");
        action_id
    }
}
