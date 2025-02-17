multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait Storage {
    #[view(getUsers)]
    #[storage_mapper("users")]
    fn users(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getTokenAmount)]
    #[storage_mapper("getTokenAmount")]
    fn token_amount(&self, token: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(lastGame)]
    #[storage_mapper("lastGame")]
    fn last_game(&self) -> SingleValueMapper<u64>;
}
