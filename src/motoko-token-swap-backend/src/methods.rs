use candid::{export_service, Principal};
use ic_cdk::{caller, query, update};

use crate::{ledger::Ledger, types::FromResult};

#[update]
pub async fn swap(from: Principal, to: Principal) -> Result<FromResult, String> {
    // initalize the FROM ledger
    let from_ledger = Ledger::new(from);

    // Check if the client has set the correct allowance on the FROM ledger
    let allowance = from_ledger.get_allowance(caller()).await?;

    // Transfer the FROM tokens to this canister under the caller's subaccount
    let transferred_amount = from_ledger
        .from_ledger_to_canister_transaction(caller(), allowance.allowance.clone())
        .await?;

    // Check the balance of the caller's subaccount on this canister
    let from_balance = from_ledger.get_balance(Some(caller())).await?;

    // initalize the TO ledger
    let to_ledger = Ledger::new(to);

    // Check the balance of the TO tokens on this canister
    let to_balance = to_ledger.get_balance(None).await?;

    // transfer the FROM token amount to the caller
    let to_transfer = to_ledger
        .from_canister_to_caller_transaction(caller(), transferred_amount.clone())
        .await?;

    Ok(FromResult {
        allowance: allowance.allowance,
        transferred_amount,
        subaccount_balance: from_balance,
        to_balance,
        to_transfer,
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
