elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait EventModule {
    #[event("buy")]
    fn buy_event(
        &self,
        #[indexed] user_address: ManagedAddress,
        #[indexed] token_id: TokenIdentifier,
        #[indexed] egld_amount: BigUint,
        #[indexed] token_amount: BigUint,
        #[indexed] timestamp: u64,
    );
}