elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct Presale<M: ManagedTypeApi> {
    pub treasury_wallet: ManagedAddress<M>,
    pub token_id: TokenIdentifier<M>,
    pub egld_price_rate: BigUint<M>,
    pub token_price_rate: BigUint<M>,

    pub start_timestamp: u64,
    pub end_timestamp: u64,
    
    pub token_sale_amount: BigUint<M>,
    pub total_bought_amount_in_egld: BigUint<M>,
    pub total_bought_amount_in_esdt: BigUint<M>,

    pub pack_amounts: ManagedVec<M, BigUint<M>>,
    pub bonus_percentages: ManagedVec<M, u64>,
}