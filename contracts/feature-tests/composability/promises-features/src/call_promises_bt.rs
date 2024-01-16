dharitri_sc::imports!();
dharitri_sc::derive_imports!();

use crate::common::{self, CallbackData};

#[dharitri_sc::module]
pub trait CallPromisesBackTransfersModule: common::CommonModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    fn forward_promise_retrieve_funds_back_transfers(
        &self,
        to: ManagedAddress,
        token: MoaOrDctTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let gas_limit = self.blockchain().get_gas_left() - 20_000_000;
        self.vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount)
            .with_gas_limit(gas_limit)
            .async_call_promise()
            .with_callback(self.callbacks().retrieve_funds_back_transfers_callback())
            .with_extra_gas_for_callback(10_000_000)
            .register_promise()
    }

    #[promises_callback]
    fn retrieve_funds_back_transfers_callback(&self) {
        let back_transfers = self.blockchain().get_back_transfers();
        let moa_transfer = back_transfers.total_moa_amount;

        if moa_transfer != BigUint::zero() {
            let moa_token_id = MoaOrDctTokenIdentifier::moa();
            self.retrieve_funds_callback_event(&moa_token_id, 0, &moa_transfer);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: moa_token_id,
                token_nonce: 0,
                token_amount: moa_transfer,
                args: ManagedVec::new(),
            });
        }

        for dct_transfer in &back_transfers.dct_payments {
            let (token, nonce, payment) = dct_transfer.into_tuple();
            let dct_token_id = MoaOrDctTokenIdentifier::dct(token);
            self.retrieve_funds_callback_event(&dct_token_id, nonce, &payment);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: dct_token_id,
                token_nonce: nonce,
                token_amount: payment,
                args: ManagedVec::new(),
            });
        }
    }
}
