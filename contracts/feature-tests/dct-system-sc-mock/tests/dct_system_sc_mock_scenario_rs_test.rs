use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/dct-system-sc-mock");

    blockchain.register_contract(
        "file:output/dct-system-sc-mock.wasm",
        dct_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn dct_system_sc_rs() {
    world().run("scenarios/dct_system_sc.scen.json");
}
