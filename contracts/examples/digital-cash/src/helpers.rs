dharitri_sc::imports!();

use crate::{
    constants::*,
    deposit_info::{DepositInfo, Fee},
    storage,
};
#[dharitri_sc::module]
pub trait HelpersModule: storage::StorageModule {
    fn send_fee_to_address(&self, fee: &MoaOrDctTokenPayment, address: &ManagedAddress) {
        if fee.token_identifier == MoaOrDctTokenIdentifier::moa() {
            self.send().direct_moa(address, &fee.amount);
        } else {
            let dct_fee = fee.clone().unwrap_dct();
            self.send()
                .direct_dct(address, &dct_fee.token_identifier, 0, &dct_fee.amount);
        }
    }

    fn get_num_token_transfers(
        &self,
        moa_value: &BigUint,
        dct_transfers: &ManagedVec<DctTokenPayment>,
    ) -> usize {
        let mut amount = dct_transfers.len();
        if moa_value > &0 {
            amount += 1;
        }

        amount
    }

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }

    fn get_fee_for_token(&self, token: &MoaOrDctTokenIdentifier) -> BigUint {
        require!(
            self.whitelisted_fee_tokens().contains(token),
            "invalid fee toke provided"
        );
        let fee_token = self.fee(token);
        fee_token.get()
    }

    fn make_fund(
        &self,
        moa_payment: BigUint,
        dct_payment: ManagedVec<DctTokenPayment>,
        address: ManagedAddress,
        valability: u64,
    ) {
        let deposit_mapper = self.deposit(&address);

        deposit_mapper.update(|deposit| {
            require!(
                deposit.moa_funds == 0 && deposit.dct_funds.is_empty(),
                "key already used"
            );
            let num_tokens = self.get_num_token_transfers(&moa_payment, &dct_payment);
            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = valability;
            deposit.expiration_round = self.get_expiration_round(valability);
            deposit.dct_funds = dct_payment;
            deposit.moa_funds = moa_payment;
        });
    }

    fn check_fees_cover_number_of_tokens(
        &self,
        num_tokens: usize,
        fee: BigUint,
        paid_fee: BigUint,
    ) {
        require!(num_tokens > 0, "amount must be greater than 0");
        require!(
            fee * num_tokens as u64 <= paid_fee,
            CANNOT_DEPOSIT_FUNDS_ERR_MSG
        );
    }

    fn update_fees(
        &self,
        caller_address: ManagedAddress,
        address: &ManagedAddress,
        payment: MoaOrDctTokenPayment,
    ) {
        self.get_fee_for_token(&payment.token_identifier);
        let deposit_mapper = self.deposit(address);
        if !deposit_mapper.is_empty() {
            deposit_mapper.update(|deposit| {
                require!(
                    deposit.depositor_address == caller_address,
                    "invalid depositor address"
                );
                require!(
                    deposit.fees.value.token_identifier == payment.token_identifier,
                    "can only have 1 type of token as fee"
                );
                deposit.fees.value.amount += payment.amount;
            });
            return;
        }

        let new_deposit = DepositInfo {
            depositor_address: caller_address,
            dct_funds: ManagedVec::new(),
            moa_funds: BigUint::zero(),
            valability: 0,
            expiration_round: 0,
            fees: Fee {
                num_token_to_transfer: 0,
                value: payment,
            },
        };
        deposit_mapper.set(new_deposit);
    }
}
