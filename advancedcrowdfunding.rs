#![no_std]

use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait CrowdfundingSc {
    #[init]
    fn init(&self, target: BigUint, deadline: u64) {
        require!(target > 0, "Target must be more than 0");
        self.target().set(target);

        require!(
            deadline > self.get_current_time(),
            "Deadline can't be in the past"
        );
        self.deadline().set(deadline);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn set_limit(&self, limit: BigUint) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address(),
            "only owner can set funding limit"
        );
        self.limit().set(limit);
    }

    #[endpoint]
    fn set_limit_per_donor(&self, limit_per_donor: BigUint) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address(),
            "only owner can set the limit per donor"
        );
        self.limit_per_donor().set(limit_per_donor);
    }

    #[endpoint]
    fn set_minimum_per_donation(&self, minimum_per_donation: BigUint) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address(),
            "only owner can set the minimum per donation"
        );
        self.minimum_per_donation().set(minimum_per_donation);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn fund(&self) {
        
        let current_time = self.blockchain().get_block_timestamp();
        require!(
            current_time < self.deadline().get(),
            "cannot fund after deadline"
        );

        let payment = self.call_value().egld().clone_value();

        // Use a clone of `payment` here so that we can still use `payment` later.
        let proposed_balance = self.get_current_funds() + payment.clone();

        if !self.minimum_per_donation().is_empty(){
            require!(
                proposed_balance <= self.limit().get(),
                "cannot exceed the maximum contribution limit"
            );
        }

        if !self.minimum_per_donation().is_empty(){
            require!(
                payment >= self.minimum_per_donation().get(),
                "cannot accept donations below the minimum contribution limit"
            );
        }

        let caller = self.blockchain().get_caller();
        let deposited_amount = self.deposit(&caller).get();
        let proposed_amount = deposited_amount + payment;

        if !self.limit_per_donor().is_empty(){
            require!(
                proposed_amount <= self.limit_per_donor().get(),
                "cannot exceed the maximum contribution limit per donor"
            );
        }

        self.deposit(&caller).set(proposed_amount);
    }

    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::FundingPeriod => sc_panic!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let sc_balance = self.get_current_funds();
                self.send().direct_egld(&caller, &sc_balance);
            }
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0u32 {
                    self.deposit(&caller).clear();
                    self.send().direct_egld(&caller, &deposit);
                }
            }
        }
    }

    #[view]
    fn status(&self) -> Status {
        if self.get_current_time() <= self.deadline().get() {
            Status::FundingPeriod
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    // private

    fn get_current_time(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    // storage

    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getLimit)]
    #[storage_mapper("limit")]
    fn limit(&self) -> SingleValueMapper<BigUint>;

    #[view(getLimit_per_donor)]
    #[storage_mapper("limit_per_donor")]
    fn limit_per_donor(&self) -> SingleValueMapper<BigUint>;

    #[view(getMinimum_per_donation)]
    #[storage_mapper("minimum_per_donation")]
    fn minimum_per_donation(&self) -> SingleValueMapper<BigUint>;


}
