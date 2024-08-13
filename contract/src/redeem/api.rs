use near_contract_standards::non_fungible_token::{core::NonFungibleTokenCore, TokenId};
use near_sdk::{env, ext_contract, json_types::U128, near, require, serde_json, PromiseError, PromiseOrValue};
use sweat_booster_model::{
    api::RedeemApi,
    event::{emit, EventKind, RedeemData},
    model::BoosterExtra,
};

use crate::{burn::api::NonFungibleTokenBurn, mint::model::ExtraExtractor, Contract, ContractExt};

#[near]
impl RedeemApi for Contract {
    fn redeem(&mut self, token_id: TokenId) -> PromiseOrValue<U128> {
        let account_id = env::predecessor_account_id();

        self.assert_owner(&account_id, &token_id);

        let token = self.tokens.nft_token(token_id.clone()).expect("Token not found");

        let BoosterExtra::BalanceBooster(mut extra) = token.get_extra();
        require!(extra.is_redeemable, "Redeem is in progress");

        extra.is_redeemable = false;
        let amount = extra.denomination;

        self.update_extra(token, BoosterExtra::BalanceBooster(extra));

        self.transfer(&account_id, token_id, amount)
    }
}

#[ext_contract(ext_self)]
trait Callbacks {
    fn on_redeem_transfer(
        &mut self,
        #[callback_result] result: Result<(), PromiseError>,
        token_id: TokenId,
    ) -> PromiseOrValue<U128>;
}

#[near]
impl Callbacks for Contract {
    #[private]
    fn on_redeem_transfer(
        &mut self,
        #[callback_result] result: Result<(), PromiseError>,
        token_id: TokenId,
    ) -> PromiseOrValue<U128> {
        if result.is_ok() {
            let metadata = self.tokens.burn(token_id.clone());

            let extra = metadata.extra.expect("Metadata doesn't contain extra");
            let BoosterExtra::BalanceBooster(extra) =
                serde_json::from_str::<BoosterExtra>(extra.as_str()).expect("Failed to parse extra");

            emit(EventKind::Redeem(RedeemData {
                token_id,
                denomination: extra.denomination.into(),
            }));

            return PromiseOrValue::Value(extra.denomination.into());
        }

        let token = self.tokens.nft_token(token_id.clone()).expect("Token not found");

        let BoosterExtra::BalanceBooster(mut extra) = token.get_extra();
        extra.is_redeemable = true;

        self.update_extra(token, BoosterExtra::BalanceBooster(extra));

        PromiseOrValue::Value(0.into())
    }
}

#[cfg(not(test))]
mod prod {
    use near_contract_standards::non_fungible_token::TokenId;
    use near_sdk::{env, json_types::U128, AccountId, Gas, Promise, PromiseOrValue};

    use crate::{
        common::{
            asserts::assert_enough_gas,
            ft_interface::{FtTransfer, GAS_FOR_FT_TRANSFER},
        },
        redeem::api::ext_self,
        Contract,
    };

    const GAS_FOR_AFTER_REDEEM: Gas = Gas::from_tgas(10);

    impl Contract {
        pub(crate) fn transfer(
            &mut self,
            receiver_id: &AccountId,
            token_id: TokenId,
            amount: u128,
        ) -> PromiseOrValue<U128> {
            assert_enough_gas(GAS_FOR_FT_TRANSFER.checked_add(GAS_FOR_AFTER_REDEEM).unwrap());

            Promise::new(self.ft_account_id.clone())
                .ft_transfer(receiver_id, amount, None)
                .then(
                    ext_self::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_AFTER_REDEEM)
                        .on_redeem_transfer(token_id),
                )
                .into()
        }
    }
}

#[cfg(test)]
mod test {
    use near_contract_standards::non_fungible_token::TokenId;
    use near_sdk::{json_types::U128, AccountId, PromiseOrValue};

    use crate::{common::test_data::get_test_future_result, redeem::api::Callbacks, Contract};

    impl Contract {
        pub(crate) fn transfer(&mut self, _: &AccountId, token_id: TokenId, _: u128) -> PromiseOrValue<U128> {
            self.on_redeem_transfer(get_test_future_result(), token_id)
        }
    }
}
