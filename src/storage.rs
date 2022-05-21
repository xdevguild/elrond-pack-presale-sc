elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait StorageModule {
    //
    #[view(getTreasuryWallet)]
    #[storage_mapper("treasury_wallet")]
    fn treasury_wallet(&self) -> SingleValueMapper<ManagedAddress>;

    #[only_owner]
    #[endpoint(setTreasuryWallet)]
    fn set_treasury_wallet(&self, treasury_wallet: ManagedAddress) {
        self.treasury_wallet().set(&treasury_wallet);
    }

    //
    #[view(getTokenId)]
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[only_owner]
    #[endpoint(setTokenId)]
    fn set_token_id(&self, token_id: TokenIdentifier) {
        self.token_id().set(&token_id);
    }

    //
    #[view(getEgldPriceRate)]
    #[storage_mapper("egld_price_rate")]
    fn egld_price_rate(&self) -> SingleValueMapper<BigUint>;

    #[view(getTokenPriceRate)]
    #[storage_mapper("token_price_rate")]
    fn token_price_rate(&self) -> SingleValueMapper<BigUint>;

    #[only_owner]
    #[endpoint(setPriceRates)]
    fn set_price_rates(&self, egld_price_rate: BigUint, token_price_rate: BigUint) {
        self.egld_price_rate().set(egld_price_rate);
        self.token_price_rate().set(token_price_rate);
    }

    //
    #[view(getStartTimestamp)]
    #[storage_mapper("start_timestamp")]
    fn start_timestamp(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setStartTimestamp)]
    fn set_start_timestamp(&self, start_timestamp: u64) {
        self.start_timestamp().set(&start_timestamp);
    }

    //
    #[view(getEndTimestamp)]
    #[storage_mapper("end_timestamp")]
    fn end_timestamp(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setEndTimestamp)]
    fn set_end_timestamp(&self, end_timestamp: u64) {
        self.end_timestamp().set(&end_timestamp);
    }

    //
    #[view(getBonusPercentages)]
    #[storage_mapper("bonus_percentages")]
    fn bonus_percentages(&self) -> MapMapper<BigUint, u64>;

    #[only_owner]
    #[endpoint(addBonusPercentages)]
    fn add_bonus_percentages(
        &self,
        items: MultiValueEncoded<MultiValue2<BigUint, u64>>
    ) {
        for item in items.into_iter() {
            let (amount, bonus_percentage) = item.into_tuple();
            self.bonus_percentages().insert(amount, bonus_percentage);
        }
    }

    #[only_owner]
    #[endpoint(clearBonusPercentages)]
    fn clear_bonus_percentages(&self) {
        self.bonus_percentages().clear();
    }

    //
    #[view(getTotalBoughtAmountInEgld)]
    #[storage_mapper("total_bought_amount_in_egld")]
    fn total_bought_amount_in_egld(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalBoughtAmountInEsdt)]
    #[storage_mapper("total_bought_amount_in_esdt")]
    fn total_bought_amount_in_esdt(&self) -> SingleValueMapper<BigUint>;

    #[view(getBoughtAmountPerWallet)]
    #[storage_mapper("bought_amount_per_wallet")]
    fn bought_amount_per_wallet(&self) -> MapMapper<ManagedAddress, BigUint>;
}