dharitri_sc::imports!();

/// Not directly related to promises, but this contract already has the setup for VM 1.5.
#[dharitri_sc::module]
pub trait BackTransfersFeatureModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    fn forward_sync_retrieve_funds_bt(
        &self,
        to: ManagedAddress,
        token: MoaOrDctTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let ((), back_transfers) = self
            .vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount)
            .execute_on_dest_context_with_back_transfers::<()>();

        require!(
            back_transfers.dct_payments.len() == 1 || back_transfers.total_moa_amount != 0,
            "Only one DCT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_moa_amount,
            &back_transfers.dct_payments.into_multi_value(),
        );
    }

    #[endpoint]
    fn forward_sync_retrieve_funds_bt_twice(
        &self,
        to: ManagedAddress,
        token: MoaOrDctTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let ((), back_transfers) = self
            .vault_proxy()
            .contract(to.clone())
            .retrieve_funds(token.clone(), token_nonce, amount.clone())
            .execute_on_dest_context_with_back_transfers::<()>();

        require!(
            back_transfers.dct_payments.len() == 1 || back_transfers.total_moa_amount != 0,
            "Only one DCT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_moa_amount,
            &back_transfers.dct_payments.into_multi_value(),
        );

        let ((), back_transfers) = self
            .vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount)
            .execute_on_dest_context_with_back_transfers::<()>();

        require!(
            back_transfers.dct_payments.len() == 1 || back_transfers.total_moa_amount != 0,
            "Only one DCT payment expected"
        );

        self.back_transfers_event(
            &back_transfers.total_moa_amount,
            &back_transfers.dct_payments.into_multi_value(),
        );
    }

    #[event("back_tranfers")]
    fn back_transfers_event(
        &self,
        #[indexed] moa_value: &BigUint,
        #[indexed] multi_dct: &MultiValueEncoded<DctTokenPaymentMultiValue>,
    );
}
