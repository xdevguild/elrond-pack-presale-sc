elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const TOTAL_PERCENTAGE: u64 = 10000u64;

#[elrond_wasm::module]
pub trait LogicModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
    #[payable("EGLD")]
    #[endpoint(buy)]
    fn buy(&self, #[payment_amount] payment_amount: BigUint) {
        self.require_activation();

        require!(
            self.bonus_percentages().contains_key(&payment_amount),
            "invalid pack amount"
        );

        let caller = self.blockchain().get_caller();
        let token_id = self.token_id().get();
        let bonus_percentage = self.bonus_percentages().get(&payment_amount).unwrap();

        let buy_amount = payment_amount.clone() * &self.token_price_rate().get() * (TOTAL_PERCENTAGE + bonus_percentage) / &self.egld_price_rate().get() / TOTAL_PERCENTAGE;

        require!(
            buy_amount <= self.blockchain().get_sc_balance(&token_id, 0),
            "not enough tokens in smart contract"
        );

        self.total_bought_amount_in_egld().update(|v| *v += &payment_amount);
        self.total_bought_amount_in_esdt().update(|v| *v += &buy_amount);

        self.send().direct(
            &caller,
            &token_id,
            0,
            &buy_amount,
            &[]
        );
        self.send().direct(
            &self.treasury_wallet().get(),
            &TokenIdentifier::egld(),
            0,
            &payment_amount,
            b"IDO treasury"
        );

        self.buy_event(
            caller,
            token_id,
            payment_amount,
            buy_amount,
            self.blockchain().get_block_timestamp()
        );
    }

    #[inline]
    fn require_activation(&self) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            current_timestamp >= self.start_timestamp().get(),
            "sale is not opened"
        );
        require!(
            current_timestamp < self.end_timestamp().get(),
            "sale is closed"
        );
    }
}