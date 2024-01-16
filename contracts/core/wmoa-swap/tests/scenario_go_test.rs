use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn unwrap_moa_go() {
    world().run("scenarios/unwrap_moa.scen.json");
}

#[test]
fn wrap_moa_go() {
    world().run("scenarios/wrap_moa.scen.json");
}
