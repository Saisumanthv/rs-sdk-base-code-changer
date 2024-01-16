use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_moa_go() {
    world().run("scenarios/claim-moa.scen.json");
}

#[test]
fn claim_dct_go() {
    world().run("scenarios/claim-dct.scen.json");
}

#[test]
fn claim_fees_go() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_multi_dct_go() {
    world().run("scenarios/claim-multi-dct.scen.json");
}

#[test]
fn forward_go() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_moa_and_dct_go() {
    world().run("scenarios/fund-moa-and-dct.scen.json");
}

#[test]
fn set_accounts_go() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn whitelist_blacklist_fee_token_go() {
    world().run("scenarios/whitelist-blacklist-fee-tokens.scen.json");
}

#[test]
fn pay_fee_and_fund_dct_go() {
    world().run("scenarios/pay-fee-and-fund-dct.scen.json");
}

#[test]
fn pay_fee_and_fund_moa_go() {
    world().run("scenarios/pay-fee-and-fund-moa.scen.json");
}

#[test]
fn withdraw_moa_go() {
    world().run("scenarios/withdraw-moa.scen.json");
}

#[test]
fn withdraw_dct_go() {
    world().run("scenarios/withdraw-dct.scen.json");
}

#[test]
fn withdraw_multi_dct_go() {
    world().run("scenarios/withdraw-multi-dct.scen.json");
}
