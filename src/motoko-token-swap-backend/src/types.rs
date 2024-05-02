use candid::{CandidType, Nat};

#[derive(Clone, Debug, CandidType)]
pub struct FromResult {
    pub allowance: Nat,
    pub transferred_amount: Nat,
    pub subaccount_balance: Nat,
    pub to_balance: Nat,
    pub to_transfer: Nat,
}
