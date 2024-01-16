use crate::{
    api::{use_raw_handle, StaticVarApiImpl},
    codec::TopDecodeMulti,
};

use crate::{
    api::{BlockchainApiImpl, CallTypeApi},
    contract_base::SendRawWrapper,
    formatter::SCLowerHex,
    io::{ArgErrorHandler, ArgId, ManagedResultArgLoader},
    types::{
        BigUint, DctTokenPayment, ManagedBuffer, ManagedBufferCachedBuilder, ManagedType,
        ManagedVec,
    },
};

use super::{AsyncCall, ContractCallNoPayment, ContractCallWithMoa};
use crate::api::managed_types::handles::HandleConstraints;

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
pub(super) const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;

/// In case of `transfer_execute`, we leave by default a little gas for the calling transaction to finish.
pub(super) const TRANSFER_EXECUTE_DEFAULT_LEFTOVER: u64 = 100_000;

impl<SA, OriginalResult> ContractCallWithMoa<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn resolve_gas_limit(&self) -> u64 {
        if self.basic.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            SA::blockchain_api_impl().get_gas_left()
        } else {
            self.basic.explicit_gas_limit
        }
    }

    #[inline]
    pub fn get_back_transfers(&self) -> (BigUint<SA>, ManagedVec<SA, DctTokenPayment<SA>>) {
        let dct_transfer_value_handle: SA::BigIntHandle =
            use_raw_handle(SA::static_var_api_impl().next_handle());
        let call_value_handle: SA::BigIntHandle =
            use_raw_handle(SA::static_var_api_impl().next_handle());

        SA::blockchain_api_impl().managed_get_back_transfers(
            dct_transfer_value_handle.get_raw_handle(),
            call_value_handle.get_raw_handle(),
        );

        (
            BigUint::from_raw_handle(call_value_handle.get_raw_handle()),
            ManagedVec::from_raw_handle(dct_transfer_value_handle.get_raw_handle()),
        )
    }

    pub fn to_call_data_string(&self) -> ManagedBuffer<SA> {
        let mut result = ManagedBufferCachedBuilder::default();
        result.append_managed_buffer(&self.basic.function_call.function_name);
        for arg in self.basic.function_call.arg_buffer.raw_arg_iter() {
            result.append_bytes(b"@");
            SCLowerHex::fmt(&*arg, &mut result);
        }
        result.into_managed_buffer()
    }

    pub(super) fn async_call(self) -> AsyncCall<SA> {
        AsyncCall {
            to: self.basic.to,
            moa_payment: self.moa_payment,
            function_call: self.basic.function_call,
            callback_call: None,
        }
    }

    #[cfg(feature = "promises")]
    pub(super) fn async_call_promise(self) -> super::AsyncCallPromises<SA> {
        super::AsyncCallPromises {
            to: self.basic.to,
            moa_payment: self.moa_payment,
            function_call: self.basic.function_call,
            explicit_gas_limit: self.basic.explicit_gas_limit,
            extra_gas_for_callback: 0,
            callback_call: None,
        }
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    pub(super) fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.moa_payment,
            &self.basic.function_call.function_name,
            &self.basic.function_call.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    pub(super) fn execute_on_dest_context_readonly<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_readonly_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.basic.function_call.function_name,
            &self.basic.function_call.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    pub(super) fn execute_on_same_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_same_context_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.moa_payment,
            &self.basic.function_call.function_name,
            &self.basic.function_call.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }
}

impl<SA, OriginalResult> ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) fn resolve_gas_limit_with_leftover(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            let mut gas_left = SA::blockchain_api_impl().get_gas_left();
            if gas_left > TRANSFER_EXECUTE_DEFAULT_LEFTOVER {
                gas_left -= TRANSFER_EXECUTE_DEFAULT_LEFTOVER;
            }
            gas_left
        } else {
            self.explicit_gas_limit
        }
    }

    pub(super) fn transfer_execute_moa(self, moa_payment: BigUint<SA>) {
        let gas_limit = self.resolve_gas_limit_with_leftover();

        let _ = SendRawWrapper::<SA>::new().direct_moa_execute(
            &self.to,
            &moa_payment,
            gas_limit,
            &self.function_call.function_name,
            &self.function_call.arg_buffer,
        );
    }

    pub(super) fn transfer_execute_single_dct(self, payment: DctTokenPayment<SA>) {
        let gas_limit = self.resolve_gas_limit_with_leftover();

        if payment.token_nonce == 0 {
            // fungible DCT
            let _ = SendRawWrapper::<SA>::new().transfer_dct_execute(
                &self.to,
                &payment.token_identifier,
                &payment.amount,
                gas_limit,
                &self.function_call.function_name,
                &self.function_call.arg_buffer,
            );
        } else {
            // non-fungible/semi-fungible DCT
            let _ = SendRawWrapper::<SA>::new().transfer_dct_nft_execute(
                &self.to,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
                gas_limit,
                &self.function_call.function_name,
                &self.function_call.arg_buffer,
            );
        }
    }

    pub(super) fn transfer_execute_multi_dct(
        self,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        let _ = SendRawWrapper::<SA>::new().multi_dct_transfer_execute(
            &self.to,
            &payments,
            gas_limit,
            &self.function_call.function_name,
            &self.function_call.arg_buffer,
        );
    }

    pub(super) fn transfer_execute_dct(self, payments: ManagedVec<SA, DctTokenPayment<SA>>) {
        match payments.len() {
            0 => self.transfer_execute_moa(BigUint::zero()),
            1 => self.transfer_execute_single_dct(payments.get(0)),
            _ => self.transfer_execute_multi_dct(payments),
        }
    }
}

fn decode_result<SA, RequestedResult>(
    raw_result: ManagedVec<SA, ManagedBuffer<SA>>,
) -> RequestedResult
where
    SA: CallTypeApi + 'static,
    RequestedResult: TopDecodeMulti,
{
    let mut loader = ManagedResultArgLoader::new(raw_result);
    let arg_id = ArgId::from(&b"sync result"[..]);
    let h: ArgErrorHandler<SA> = ArgErrorHandler::<SA>::from(arg_id);
    let Ok(result) = RequestedResult::multi_decode_or_handle_err(&mut loader, h);
    result
}
