multiversx_sc::derive_imports!();
multiversx_sc::imports!();

// #[type_abi]
// #[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug)]
// pub struct DelegationContractData<M: ManagedTypeApi> {
//     pub admin_address: ManagedAddress<M>,
//     pub total_staked: BigUint<M>,
//     pub delegation_contract_cap: BigUint<M>,
//     pub nr_nodes: u64,
//     pub apy: u64,
//     pub total_staked_from_ls_contract: BigUint<M>,
//     pub total_unstaked_from_ls_contract: BigUint<M>,
//     pub eligible: bool,
//     pub pending_staking_callback_amount: BigUint<M>,
//     pub pending_unstaking_callback_amount: BigUint<M>,
// }

// Example of implementing extra functions for a struct to use the data part of it
// impl<M: ManagedTypeApi> DelegationContractData<M> {
//     pub fn get_total_amount_with_pending_callbacks(&self) -> BigUint<M> {
//         let total = &self.total_staked_from_ls_contract + &self.pending_staking_callback_amount;
//         if total > self.pending_unstaking_callback_amount {
//             total - &self.pending_unstaking_callback_amount
//         } else {
//             BigUint::zero()
//         }
//     }
// }

// #[type_abi]
// #[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug)]
// pub struct UnstakeTokenAttributes {
//     pub unstake_epoch: u64,
//     pub unbond_epoch: u64,
// }

// Example of creating a struct via a new function
// impl UnstakeTokenAttributes {
//     pub fn new(unstake_epoch: u64, unbond_epoch: u64) -> Self {
//         UnstakeTokenAttributes {
//             unstake_epoch,
//             unbond_epoch,
//         }
//     }
// }

// Example of an quick Enum
// #[type_abi]
// #[derive(TopEncode, TopDecode, PartialEq, Eq, Copy, Clone, Debug)]
// pub enum State {
//     Running,
//     Stopped,
// }