use dharitri_sc::api::DCT_MULTI_TRANSFER_FUNC_NAME;
use dharitri_sc_scenario::scenario_model::ScCallStep;
use num_traits::Zero;

#[test]
fn test_contract_call_multi_dct() {
    let tx = ScCallStep::new()
        .from("address:sender")
        .to("address:recipient")
        .dct_transfer("str:WMOA-abcdef", 0, 10u32)
        .dct_transfer("str:USDC-abcdef", 0, 11u32);

    let cc = tx.tx.to_contract_call();

    assert_eq!(
        cc.basic.function_call.function_name.to_vec(),
        DCT_MULTI_TRANSFER_FUNC_NAME.as_bytes().to_vec(),
    );
    assert_eq!(
        cc.to_call_data_string().to_string(),
        "MultiDCTNFTTransfer@726563697069656e745f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f@02@5745474c442d616263646566@@0a@555344432d616263646566@@0b",
    );
    assert!(tx.tx.moa_value.value.is_zero());
    assert_eq!(tx.tx.from.value, cc.basic.to.to_address());
}
