#![no_std]

use multiversx_sc::imports::*;
use multiversx_sc_modules::{only_admin, pause};

pub mod contexts;
pub mod events;
pub mod storage;
pub mod constants;
pub mod errors;
pub mod structs;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Template:
    storage::Storage + events::Events + pause::PauseModule + only_admin::OnlyAdminModule
{
    #[init]
    fn init(&self) {
        let deployer = self.blockchain().get_caller();
        self.add_admin(deployer);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
