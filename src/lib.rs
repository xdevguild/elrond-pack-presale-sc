#![no_std]
#![feature(generic_associated_types)]
#![feature(let_chains)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod event;
mod logic;
mod storage;
mod state;

use state::{Presale};

#[elrond_wasm::derive::contract]
pub trait ElrondPackPresale:
    logic::LogicModule
    + storage::StorageModule
    + event::EventModule
{
    #[init]
    fn init(
        &self,
        treasury_wallet: ManagedAddress,
        token_id: TokenIdentifier,
        egld_price_rate: BigUint,
        token_price_rate: BigUint,
        start_timestamp: u64,
        end_timestamp: u64,
    ) {
        self.treasury_wallet().set(treasury_wallet);
        self.token_id().set(token_id);
        self.egld_price_rate().set(egld_price_rate);
        self.token_price_rate().set(token_price_rate);
        self.start_timestamp().set(start_timestamp);
        self.end_timestamp().set(end_timestamp);
    }

    #[only_owner]
    #[endpoint(recoverToken)]
    fn recover_token(&self,
        opt_token_id: OptionalValue<TokenIdentifier>,
        opt_token_nonce: OptionalValue<u64>,
        opt_token_amount: OptionalValue<BigUint>
    ) {
        // if token_id is not given, set it to eGLD
        let token_id = match opt_token_id {
            OptionalValue::Some(v) => v,
            OptionalValue::None => TokenIdentifier::egld()
        };

        // if token_id is not given, set it to 0
        let token_nonce = match opt_token_nonce {
            OptionalValue::Some(v) => v,
            OptionalValue::None => 0,
        };

        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let token_amount = match opt_token_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_sc_balance(&token_id, 0)
        };

        self.send().direct(
            &self.blockchain().get_caller(),
            &token_id,
            token_nonce,
            &token_amount,
            &[]
        );
    }

    #[view(viewPresale)]
    fn view_presale(&self) -> Presale<Self::Api> {
        let token_sale_amount = self.blockchain().get_sc_balance(&self.token_id().get(), 0);

        let mut pack_amounts = ManagedVec::new();
        let mut bonus_percentages = ManagedVec::new();
        for (amount, bonus_percentage) in self.bonus_percentages().iter() {
            pack_amounts.push(amount);
            bonus_percentages.push(bonus_percentage);
        }

        Presale {
            treasury_wallet: self.treasury_wallet().get(),
            token_id: self.token_id().get(),
            egld_price_rate: self.egld_price_rate().get(),
            token_price_rate: self.token_price_rate().get(),
            start_timestamp: self.start_timestamp().get(),
            end_timestamp: self.end_timestamp().get(),
            token_sale_amount: token_sale_amount,
            total_bought_amount_in_egld: self.total_bought_amount_in_egld().get(),
            total_bought_amount_in_esdt: self.total_bought_amount_in_esdt().get(),
        
            pack_amounts,
            bonus_percentages,
        }
    }
}