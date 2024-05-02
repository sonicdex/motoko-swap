use candid::{export_service, Nat, Principal};
use ic_cdk::{caller, query, update};

use crate::{ledger::Ledger, types::FromResult};

#[update]
pub async fn swap(from: Principal, to: Principal) -> Result<FromResult, String> {
    // initalize the FROM ledger
    let from_ledger = Ledger::new(from);

    // Check if the client has set the correct allowance on the FROM ledger
    let allowance = from_ledger.get_allowance(caller()).await?;

    // Transfer the FROM tokens to this canister under the caller's subaccount
    let _blockheight = from_ledger
        .from_ledger_to_canister_transaction(caller(), allowance.allowance.clone())
        .await?;

    // Check the balance of the caller's subaccount on this canister
    let from_balance = from_ledger.get_balance(Some(caller())).await?;

    // initalize the TO ledger
    let to_ledger = Ledger::new(to);

    // transfer the FROM token amount to the caller
    let to_transfer = to_ledger
        .from_canister_to_caller_transaction(caller(), from_balance.clone())
        .await?;

    Ok(FromResult {
        from_allowance: allowance.allowance,
        caller_canister_balance: from_balance,
        to_transfer_amount: to_transfer,
    })
}

#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    export_service!();
    __export_service()
}

#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // let dir = dir.parent().unwrap();
    write(
        dir.join(format!("motoko-token-swap-backend.did")),
        __export_did_tmp_(),
    )
    .expect("Write failed.");
}
