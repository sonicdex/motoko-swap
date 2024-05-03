use candid::{CandidType, Nat};

#[derive(Clone, Debug, CandidType)]
pub struct FromResult {
    pub from_allowance: Nat,
    pub caller_canister_balance: Nat,
    pub to_transfer_amount: Nat,
}
