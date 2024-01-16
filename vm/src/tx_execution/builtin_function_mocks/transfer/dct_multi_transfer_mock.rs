use crate::{
    tx_execution::{builtin_function_names::DCT_MULTI_TRANSFER_FUNC_NAME, BlockchainVMRef},
    tx_mock::TxLog,
    types::{top_decode_u64, top_encode_u64},
};

use crate::{
    tx_execution::BuiltinFunctionDctTransferInfo,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
    types::VMAddress,
};

use super::{
    super::BuiltinFunction,
    transfer_common::{
        adjust_call_type, execute_transfer_builtin_func, extract_transfer_info,
        push_func_name_if_necessary, push_transfer_bytes, ParsedTransferBuiltinFunCall,
        RawDctTransfer,
    },
};

pub struct DCTMultiTransfer;

impl BuiltinFunction for DCTMultiTransfer {
    fn name(&self) -> &str {
        DCT_MULTI_TRANSFER_FUNC_NAME
    }

    fn extract_dct_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDctTransferInfo {
        if let Ok(parsed_tx) = try_parse_input(tx_input) {
            extract_transfer_info(parsed_tx)
        } else {
            BuiltinFunctionDctTransferInfo::empty(tx_input)
        }
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        vm: &BlockchainVMRef,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                let log = build_log(&tx_input, &parsed_tx);
                execute_transfer_builtin_func(vm, parsed_tx, tx_input, tx_cache, log, f)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message);
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn build_log(tx_input: &TxInput, call: &ParsedTransferBuiltinFunCall) -> TxLog {
    let call_type = adjust_call_type(tx_input.call_type, call);
    let mut topics = Vec::new();
    push_transfer_bytes(&call.raw_dct_transfers, &mut topics);
    topics.push(call.destination.to_vec());

    let mut data = vec![
        call_type.to_log_bytes(),
        DCT_MULTI_TRANSFER_FUNC_NAME.into(),
        call.destination.to_vec(),
        top_encode_u64(call.raw_dct_transfers.len() as u64),
    ];
    push_transfer_bytes(&call.raw_dct_transfers, &mut data);
    push_func_name_if_necessary(call_type, &call.func_name, &mut data);

    TxLog {
        address: tx_input.from.clone(),
        endpoint: DCT_MULTI_TRANSFER_FUNC_NAME.into(),
        topics,
        data,
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 2 {
        return Err("MultiDCTNFTTransfer too few arguments");
    }
    if tx_input.to != tx_input.from {
        // TODO: not sure what the real error message would be, certainly not this
        return Err("MultiDCTNFTTransfer expects that to == from");
    }

    let mut arg_index = 0;
    let destination_bytes = tx_input.args[arg_index].as_slice();
    let destination = VMAddress::from_slice(destination_bytes);
    arg_index += 1;
    let num_payments = top_decode_u64(tx_input.args[arg_index].as_slice()) as usize;
    arg_index += 1;

    if tx_input.args.len() < 2 + num_payments * 3 {
        return Err("MultiDCTNFTTransfer too few arguments");
    }

    let mut raw_dct_transfers = Vec::new();
    for _ in 0..num_payments {
        let token_identifier = tx_input.args[arg_index].clone();
        arg_index += 1;
        let nonce_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;
        let value_bytes = tx_input.args[arg_index].clone();
        arg_index += 1;

        raw_dct_transfers.push(RawDctTransfer {
            token_identifier: token_identifier.clone(),
            nonce_bytes,
            value_bytes,
        });
    }

    let func_name = tx_input.func_name_from_arg_index(arg_index);
    arg_index += 1;
    let args = if tx_input.args.len() > arg_index {
        tx_input.args[arg_index..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination,
        raw_dct_transfers,
        func_name,
        args,
    })
}
