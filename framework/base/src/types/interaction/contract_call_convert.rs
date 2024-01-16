use core::marker::PhantomData;

use crate::{
    api::CallTypeApi,
    contract_base::BlockchainWrapper,
    types::{BigUint, DctTokenPayment, ManagedVec},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCallWithMoa};

impl<SA, OriginalResult> ContractCallWithMoa<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// If this is an DCT call, it converts it to a regular call to DCTTransfer.
    /// Async calls require this step, but not `transfer_dct_execute`.
    pub fn convert_to_dct_transfer_call(
        self,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) -> Self {
        match payments.len() {
            0 => self,
            1 => self.convert_to_single_transfer_dct_call(payments.get(0)),
            _ => self.convert_to_multi_transfer_dct_call(payments),
        }
    }

    pub(super) fn convert_to_single_transfer_dct_call(
        self,
        payment: DctTokenPayment<SA>,
    ) -> Self {
        if payment.token_nonce == 0 {
            // fungible DCT
            ContractCallWithMoa {
                basic: ContractCallNoPayment {
                    _phantom: PhantomData,
                    to: self.basic.to,
                    function_call: self
                        .basic
                        .function_call
                        .convert_to_single_transfer_fungible_call(payment),
                    explicit_gas_limit: self.basic.explicit_gas_limit,
                    _return_type: PhantomData,
                },
                moa_payment: BigUint::zero(),
            }
        } else {
            // nft transfer is sent to self, sender = receiver
            let recipient_addr = BlockchainWrapper::<SA>::new().get_sc_address();

            ContractCallWithMoa {
                basic: ContractCallNoPayment {
                    _phantom: PhantomData,
                    to: recipient_addr,
                    function_call: self
                        .basic
                        .function_call
                        .convert_to_single_transfer_nft_call(&self.basic.to, payment),
                    explicit_gas_limit: self.basic.explicit_gas_limit,
                    _return_type: PhantomData,
                },
                moa_payment: BigUint::zero(),
            }
        }
    }

    fn convert_to_multi_transfer_dct_call(
        self,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) -> Self {
        // multi transfer is sent to self, sender = receiver
        let recipient_addr = BlockchainWrapper::<SA>::new().get_sc_address();

        ContractCallWithMoa {
            basic: ContractCallNoPayment {
                _phantom: PhantomData,
                to: recipient_addr,
                function_call: self
                    .basic
                    .function_call
                    .convert_to_multi_transfer_dct_call(&self.basic.to, payments),
                explicit_gas_limit: self.basic.explicit_gas_limit,
                _return_type: PhantomData,
            },
            moa_payment: BigUint::zero(),
        }
    }
}
